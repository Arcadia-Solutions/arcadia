#!/usr/bin/env bash
# End to end test of arcadia-backup: seed, backup, mutate, backup, restore the first snapshot,
# and compare with the seeded state.
#
# Usage: tools/arcadia-backup/tests/e2e.sh docker|standard
#
# Needs docker, restic, rest-server, jq, the arcadia-backup binary (ARCADIA_BACKUP, defaults to
# target/debug/arcadia-backup) and an ssh server of THIS machine reachable as SSH_TARGET without a
# password: the "Arcadia host" is this machine, so its state can be checked locally.
# Standard mode also needs pg_dump/psql, mariadb/mariadb-dump and redis-cli.
#
# Everything runs in a throwaway directory and its own compose project, a development setup is
# left untouched.
set -euo pipefail

# the compose project is the one of the throwaway directory, never one from the caller's shell
unset COMPOSE_PROJECT_NAME COMPOSE_FILE COMPOSE_PROFILES

MODE="${1:?usage: e2e.sh docker|standard}"
REPO_ROOT="$(git rev-parse --show-toplevel)"
BIN="${ARCADIA_BACKUP:-$REPO_ROOT/target/debug/arcadia-backup}"
SSH_TARGET="${SSH_TARGET:?set SSH_TARGET, e.g. $USER@localhost}"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/arcadia-e2e-XXXXXX")"
NAME="$(basename "$WORK" | tr '[:upper:]' '[:lower:]')"
ARCADIA_DIR="$WORK/$NAME"
CONF="$WORK/arcadia-backup.yml"
PG_PORT=25432
REDIS_PORT=26379
CHEVERETO_PORT=23306
ERGO_PORT=23307
APPEND_PID=""

fail() { echo "FAIL: $*" >&2; exit 1; }
step() { echo "=== $*" >&2; }

retry() {
    local attempt=0
    until "$@" >/dev/null 2>&1; do
        attempt=$((attempt + 1))
        [ "$attempt" -lt 60 ] || fail "timed out waiting for: $*"
        sleep 1
    done
}

dc() { (cd "$ARCADIA_DIR" && docker compose "$@"); }
volume() { dc --profile '*' config --format json | jq -r --arg v "$1" '.volumes[$v].name'; }

cleanup() {
    set +e
    [ -z "$APPEND_PID" ] || kill "$APPEND_PID" 2>/dev/null
    if [ "$MODE" = docker ]; then
        dc --profile '*' down -v --remove-orphans >/dev/null 2>&1
    else
        docker rm -f "$NAME-pg" "$NAME-redis" "$NAME-chevereto" "$NAME-ergo" >/dev/null 2>&1
    fi
    # the docker runner restores as root, clean up whatever a failed run left behind
    docker run --rm -v "$WORK:/w" alpine:3 sh -c 'rm -rf /w/* /w/.[!.]*' >/dev/null 2>&1
    rm -rf "$WORK"
}
trap cleanup EXIT

# --- access to the services, per mode -------------------------------------------------------

psql_run() {
    if [ "$MODE" = docker ]; then
        dc exec -T db psql -U arcadia -d arcadia -v ON_ERROR_STOP=1 -Atq "$@"
    else
        PGPASSWORD=password psql -h 127.0.0.1 -p "$PG_PORT" -U arcadia -d arcadia -v ON_ERROR_STOP=1 -Atq "$@"
    fi
}

# maria chevereto|ergo "SQL"
maria() {
    if [ "$MODE" = docker ]; then
        dc exec -T "$1_database" sh -c 'MYSQL_PWD="$MYSQL_PASSWORD" mariadb -u"$MYSQL_USER" -N "$MYSQL_DATABASE" -e "$0"' "$2"
    else
        local port="$CHEVERETO_PORT"
        [ "$1" = ergo ] && port="$ERGO_PORT"
        MYSQL_PWD=e2e mariadb -h 127.0.0.1 -P "$port" -u e2e -N e2e -e "$2"
    fi
}

redis_cli() {
    if [ "$MODE" = docker ]; then
        dc exec -T redis redis-cli "$@"
    else
        redis-cli -h 127.0.0.1 -p "$REDIS_PORT" "$@"
    fi
}

files_volume() { [ "$1" = chevereto ] && volume chevereto_storage || volume ergo_data; }

# files_write chevereto|ergo relative/path content
files_write() {
    if [ "$MODE" = docker ]; then
        docker run --rm -v "$(files_volume "$1"):/v" alpine:3 sh -c 'mkdir -p "$(dirname "/v/$0")" && printf %s "$1" > "/v/$0"' "$2" "$3"
    else
        mkdir -p "$(dirname "$WORK/files-$1/$2")"
        printf %s "$3" > "$WORK/files-$1/$2"
    fi
}

# only the e2e files: ergo keeps writing its own database next to them
files_hash() {
    local script='cd "$0" && find . -path "./e2e*" -type f | sort | xargs sha256sum'
    if [ "$MODE" = docker ]; then
        docker run --rm -v "$(files_volume "$1"):/v" alpine:3 sh -c "$script" /v
    else
        sh -c "$script" "$WORK/files-$1"
    fi
}

# --- setup ----------------------------------------------------------------------------------

setup_docker() {
    mkdir -p "$ARCADIA_DIR"
    git -C "$REPO_ROOT" archive HEAD | tar -x -C "$ARCADIA_DIR"
    cp "$REPO_ROOT/compose.yml" "$ARCADIA_DIR/compose.yml"
    sed -e 's/^  host: arcadiadb$/  host: db/' "$REPO_ROOT/config.ci.yml" > "$ARCADIA_DIR/config.yml"
    cp "$ARCADIA_DIR/ergo/ergo-conf.yaml.example" "$ARCADIA_DIR/ergo/ergo-conf.yaml"
    cp "$ARCADIA_DIR/ergo/ergo.motd.example" "$ARCADIA_DIR/ergo/ergo.motd"
    # publish no host port: the test goes through docker exec, and a development setup on the same
    # machine may already own 6667 or 8083
    cat > "$ARCADIA_DIR/compose.override.yml" <<EOF
services:
  chevereto_php:
    ports: !override []
  ergo:
    ports: !override []
EOF
    dc up -d --wait db redis chevereto_database ergo_database
    dc up -d chevereto_php ergo
}

setup_standard() {
    mkdir -p "$ARCADIA_DIR" "$WORK/files-chevereto" "$WORK/files-ergo" "$WORK/redis"
    local pg_major
    pg_major="$(pg_dump --version | sed -E 's/[^0-9]*([0-9]+).*/\1/')"
    docker run -d --name "$NAME-pg" -e POSTGRES_USER=arcadia -e POSTGRES_PASSWORD=password -e POSTGRES_DB=arcadia \
        -p "127.0.0.1:$PG_PORT:5432" "postgres:$pg_major" >/dev/null
    for db in chevereto ergo; do
        local port="$CHEVERETO_PORT"
        [ "$db" = ergo ] && port="$ERGO_PORT"
        docker run -d --name "$NAME-$db" -e MARIADB_ROOT_PASSWORD=root -e MARIADB_DATABASE=e2e \
            -e MARIADB_USER=e2e -e MARIADB_PASSWORD=e2e -p "127.0.0.1:$port:3306" mariadb:lts >/dev/null
    done
    start_standard_redis
    printf 'e2e\n' > "$WORK/chevereto.pass"
    printf 'e2e\n' > "$WORK/ergo.pass"
    cat > "$ARCADIA_DIR/config.yml" <<EOF
database:
  host: 127.0.0.1
  port: $PG_PORT
  user: arcadia
  password: password
  name: arcadia
redis:
  host: 127.0.0.1
  port: $REDIS_PORT
  password: ""
EOF
    retry env PGPASSWORD=password psql -h 127.0.0.1 -p "$PG_PORT" -U arcadia -d arcadia -c 'SELECT 1'
    retry maria chevereto 'SELECT 1'
    retry maria ergo 'SELECT 1'
}

# as the current user, so that the restore can replace dump.rdb
start_standard_redis() {
    docker rm -f "$NAME-redis" >/dev/null 2>&1 || true
    docker run -d --name "$NAME-redis" --user "$(id -u):$(id -g)" -v "$WORK/redis:/data" \
        -p "127.0.0.1:$REDIS_PORT:6379" redis >/dev/null
    retry redis_cli PING
}

write_tool_config() {
    printf 'e2e-restic-password\n' > "$WORK/password"
    # logs how the tool starts rest-server, then runs the real one
    local real
    real="$(command -v rest-server)"
    printf '#!/bin/sh\necho "$*" >> "%s/rest-server.args"\nexec "%s" "$@"\n' "$WORK" "$real" > "$WORK/rest-server-wrapper"
    chmod +x "$WORK/rest-server-wrapper"
    local runner=binary
    [ "$MODE" = docker ] && runner=docker
    cat > "$CONF" <<EOF
ssh:
  target: $SSH_TARGET
arcadia:
  dir: $ARCADIA_DIR
  mode: $MODE
  name: e2e
restic:
  repository: $WORK/repository
  password_file: $WORK/password
  runner: $runner
  rest_server: $WORK/rest-server-wrapper
retention:
  keep_last: 10
EOF
    if [ "$MODE" = standard ]; then
        cat >> "$CONF" <<EOF
standard:
  chevereto:
    database: { port: $CHEVERETO_PORT, user: e2e, password_file: $WORK/chevereto.pass, name: e2e }
    dir: $WORK/files-chevereto
  ergo:
    database: { port: $ERGO_PORT, user: e2e, password_file: $WORK/ergo.pass, name: e2e }
    dir: $WORK/files-ergo
  redis:
    rdb_path: $WORK/redis/dump.rdb
EOF
    fi
}

# --- scenario -------------------------------------------------------------------------------

seed() {
    # big and unchanged by mutate: the second backup must not upload it again
    psql_run -c "CREATE TABLE e2e_bulk (id int PRIMARY KEY, payload text)" \
        -c "INSERT INTO e2e_bulk SELECT i, md5(i::text) || md5((i * 7)::text) FROM generate_series(1, 400000) i" \
        -c "CREATE TABLE e2e_small (id int PRIMARY KEY, payload text)" \
        -c "INSERT INTO e2e_small SELECT i, 'row ' || i FROM generate_series(1, 100) i"
    for db in chevereto ergo; do
        maria "$db" "CREATE TABLE e2e_marker (id INT PRIMARY KEY, payload TEXT); INSERT INTO e2e_marker VALUES (1, '$db one'), (2, '$db two')"
    done
    redis_cli SET e2e_key before >/dev/null
    files_write chevereto e2e/image.txt "chevereto file"
    files_write ergo e2e.txt "ergo file"
    mkdir -p "$ARCADIA_DIR/frontend/public/home"
    echo '<h1>home</h1>' > "$ARCADIA_DIR/frontend/public/home/index.html"
}

capture() {
    echo "bulk $(psql_run -c "SELECT count(*), md5(string_agg(payload, ',' ORDER BY id)) FROM e2e_bulk")"
    echo "small $(psql_run -c "SELECT count(*), md5(string_agg(payload, ',' ORDER BY id)) FROM e2e_small")"
    for db in chevereto ergo; do
        echo "$db $(maria "$db" "SELECT COUNT(*), MD5(GROUP_CONCAT(payload ORDER BY id)) FROM e2e_marker")"
        echo "$db-files $(files_hash "$db" | tr '\n' ' ')"
    done
    echo "redis $(redis_cli GET e2e_key)"
    echo "home $(sha256sum < "$ARCADIA_DIR/frontend/public/home/index.html")"
    echo "config $(sha256sum < "$ARCADIA_DIR/config.yml")"
    if [ "$MODE" = docker ]; then
        for f in compose.override.yml ergo/ergo-conf.yaml ergo/ergo.motd; do
            echo "$f $(sha256sum < "$ARCADIA_DIR/$f")"
        done
    fi
}

mutate() {
    psql_run -c "DELETE FROM e2e_small WHERE id % 2 = 0" -c "INSERT INTO e2e_small VALUES (1000, 'new')"
    maria chevereto "DELETE FROM e2e_marker WHERE id = 1"
    maria ergo "UPDATE e2e_marker SET payload = 'changed'"
    redis_cli DEL e2e_key >/dev/null
    files_write chevereto e2e/image.txt "overwritten"
    files_write chevereto e2e/new.txt "must disappear"
    files_write ergo e2e-new.txt "must disappear"
    echo '<h1>changed</h1>' > "$ARCADIA_DIR/frontend/public/home/index.html"
    echo '# mutated' >> "$ARCADIA_DIR/config.yml"
    if [ "$MODE" = docker ]; then
        echo '# mutated' >> "$ARCADIA_DIR/compose.override.yml"
        echo '# mutated' >> "$ARCADIA_DIR/ergo/ergo-conf.yaml"
        echo 'mutated' >> "$ARCADIA_DIR/ergo/ergo.motd"
    fi
}

data_added() { sed -n 's/.*data_added=\([0-9]*\).*/\1/p'; }

# every rest-server the tool started must have been append only and local
check_rest_server_args() {
    local file="$WORK/rest-server.args"
    [ -s "$file" ] || fail "the tool never started rest-server"
    local total
    total="$(wc -l < "$file" | tr -d ' ')"
    [ "$(grep -c -- '--append-only' "$file")" -eq "$total" ] || fail "a rest-server was started without --append-only: $(cat "$file")"
    [ "$(grep -c -- '--listen 127.0.0.1:' "$file")" -eq "$total" ] || fail "a rest-server was not bound to 127.0.0.1: $(cat "$file")"
    echo "rest-server invocations: $total, all with --append-only and --listen 127.0.0.1:" >&2
}

check_append_only() {
    local port=28123 err
    rest-server --path "$WORK/repository" --listen "127.0.0.1:$port" --append-only --no-auth >/dev/null 2>&1 &
    APPEND_PID=$!
    retry bash -c "exec 3<>/dev/tcp/127.0.0.1/$port"
    export RESTIC_PASSWORD_FILE="$WORK/password"
    if err="$(restic -r "rest:http://127.0.0.1:$port/" forget --keep-last 1 --prune 2>&1)"; then
        fail "forget went through an append only rest-server"
    fi
    echo "$err" | grep -qiE '403|forbidden|append' || fail "forget failed for an unrelated reason: $err"
    [ "$(restic -r "rest:http://127.0.0.1:$port/" snapshots --json | jq length)" -eq 2 ] || fail "snapshots are gone after the refused forget"
    echo "append only: forget refused, 2 snapshots intact" >&2
    kill "$APPEND_PID"
    APPEND_PID=""
}

step "setup ($MODE)"
"setup_$MODE"
write_tool_config
"$BIN" --config "$CONF" init

step "seed"
seed
before="$(capture)"

step "first backup"
first="$("$BIN" --config "$CONF" backup | data_added)"

step "mutate"
mutate
[ "$(capture)" != "$before" ] || fail "mutate changed nothing"

step "second backup"
second="$("$BIN" --config "$CONF" backup | data_added)"
echo "data added: first=$first second=$second" >&2
[ "$second" -lt $((first / 4)) ] || fail "the second backup added $second bytes, the first $first: no deduplication"
[ "$("$BIN" --config "$CONF" snapshots | tail -n +2 | wc -l)" -eq 2 ] || fail "expected 2 snapshots"

step "restore the first snapshot"
[ "$MODE" = standard ] && docker stop "$NAME-redis" >/dev/null
"$BIN" --config "$CONF" restore --snapshot 1 --yes
if [ "$MODE" = standard ]; then start_standard_redis; else retry redis_cli PING; fi

check_rest_server_args

step "compare"
after="$(capture)"
diff <(echo "$before") <(echo "$after") || fail "the restored state differs from the seeded one"

step "extract"
export RESTIC_PASSWORD_FILE="$WORK/password"
extracted="$WORK/extracted"
lines="$("$BIN" --config "$CONF" extract --snapshot 1 --target "$extracted")"
[ "$(stat -c %a "$extracted")" = 700 ] || fail "the extraction directory is not mode 700"
if [ "$MODE" = docker ]; then
    extracted_meta="$extracted/arcadia/meta/meta.json"
else
    extracted_meta="$(find "$extracted" -path '*/staging/meta.json' | head -n 1)"
fi
[ -n "$extracted_meta" ] && [ -f "$extracted_meta" ] || fail "no meta.json in the extraction"
dump_path="$(echo "$lines" | sed -n 's/^postgres\/postgres\.sql: //p')"
[ -f "$dump_path" ] || fail "the extract output does not name an extracted postgres dump: $lines"
snapshot_id="$(restic -r "$WORK/repository" snapshots --json | jq -r 'sort_by(.time) | .[0].id')"
rel="${dump_path#"$extracted"}"
cmp "$dump_path" <(restic -r "$WORK/repository" dump "$snapshot_id" "$rel") || fail "the extracted postgres dump differs from the snapshot"
if "$BIN" --config "$CONF" extract --snapshot 1 --target "$extracted" 2>/dev/null; then
    fail "a second extraction into a non-empty directory succeeded"
fi

step "append only"
check_append_only

echo "PASS ($MODE)" >&2
