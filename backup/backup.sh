#!/usr/bin/env bash
# Dumps the databases, then snapshots the dumps, the configuration files and the volumes (docker) or
# data directories (host) into the local restic repository, prunes it and copies the snapshots to
# backup.remote_repo when set. Configured by the `backup` section of config.yml, see
# docs/src/backup.md.
cd "$(dirname "$0")/.." || exit 1
. backup/lib.sh

check_current_state() {
    echo "=== Current install state ==="
    echo "Mode: $MODE"
    echo "Repo: $REPO"
    echo "Dump dir: $DUMP_DIR"
    echo

    # Postgres
    local user name host port
    user=$(config_value database user)
    name=$(config_value database name)
    host=$(config_value database host)
    port=$(config_value database port)
    echo "--- Postgres ($host:$port/$name as $user) ---"
    if PGPASSWORD=$(config_value database password) psql -h "$host" -p "$port" -U "$user" -d "$name" -qAt \
        -c "SELECT pg_size_pretty(pg_database_size('$name')), (SELECT count(*) FROM information_schema.tables WHERE table_schema='public'), (SELECT sum(n_live_tup) FROM pg_stat_user_tables)::bigint;" \
        2>&1 | tail -n 1; then
        : # success, already printed
    else
        echo "(failed to connect/query)"
    fi
    echo

    # MariaDB
    check_mariadb_state() {
        local svc="$1"
        local dbname="$2"
        local envpref="$3"
        echo "--- MariaDB $svc ---"
        if [ "$MODE" = docker ]; then
            local u p d
            eval "u=\${${envpref}_USER:-}"
            eval "p=\${${envpref}_PASSWORD:-}"
            eval "d=\${${envpref}_NAME:-}"
            [ -n "$u" ] && [ -n "$d" ] || {
                echo "(no creds configured)"
                echo
                return 0
            }
            # Try TCP probe to service
            if (exec 3<> "/dev/tcp/$svc/3306") 2>/dev/null; then
                exec 3>&- 2>/dev/null || true
                exec 3<&- 2>/dev/null || true
                local tmp
                tmp=$(mktemp "$DUMP_DIR/.state.$svc.XXXXXX")
                if MYSQL_PWD="$p" mariadb -h "$svc" -u "$u" "$d" -qAt \
                    -e "SELECT ROUND(SUM(data_length+index_length)/1024/1024,2), COUNT(*), COALESCE(SUM(table_rows),0) FROM information_schema.tables WHERE table_schema='$d';" >"$tmp" 2>&1; then
                    read -r mb cnt rows <"$tmp" || {
                        echo "(failed to parse)"
                        rm -f "$tmp"
                        echo
                        return 0
                    }
                    echo "${mb:-0} MB, ${cnt:-0} tables, ~${rows:-0} rows"
                else
                    echo "(failed to query)"
                fi
                rm -f "$tmp"
            else
                echo "(not reachable on compose network)"
            fi
        elif [ -n "$dbname" ]; then
            if mariadb -qAt -e "SELECT ROUND(SUM(data_length+index_length)/1024/1024,2), COUNT(*), COALESCE(SUM(table_rows),0) FROM information_schema.tables WHERE table_schema='$dbname';" "$dbname" >/dev/null 2>&1; then
                mariadb -qAt -e "SELECT ROUND(SUM(data_length+index_length)/1024/1024,2), COUNT(*), COALESCE(SUM(table_rows),0) FROM information_schema.tables WHERE table_schema='$dbname';" "$dbname" | while read -r mb cnt rows; do
                    echo "${mb:-0} MB, ${cnt:-0} tables, ~${rows:-0} rows"
                done
            else
                echo "(failed to connect/query)"
            fi
        else
            echo "(no db name configured)"
        fi
        echo
    }

    check_mariadb_state ergo_database "$(cfg ergo_db)" ERGO_DB
    check_mariadb_state chevereto_database "$(cfg chevereto_db)" CHEVERETO_DB
    echo "=== End state check ==="
}

dump_postgres() {
    local user name
    user=$(config_value database user)
    name=$(config_value database name)
    PGPASSWORD=$(config_value database password) pg_dump \
        -h "$(config_value database host)" -p "$(config_value database port)" \
        -U "$user" -d "$name" --no-owner --no-privileges > "$DUMP_DIR/arcadia.sql"
}

# $1: compose service, also the dump name. $2: database name (host mode). $3: env prefix for the
# per-database credentials (docker mode). Docker: dumped over the compose network as the owning
# user. A service that is not running (optional profile) is skipped; a reachable one whose dump
# fails or is empty fails the run, and nothing empty is ever stored: restore DROP+CREATEs from the
# dump and would wipe the database.
dump_mariadb() {
    if [ "$MODE" = docker ]; then
        local u p d tmp
        eval "u=\${${3}_USER:-}"; eval "p=\${${3}_PASSWORD:-}"; eval "d=\${${3}_NAME:-}"
        [ -n "$u" ] && [ -n "$d" ] || return 0
        # optional service not running => skip. The subshell opens and closes the probe socket.
        (exec 3<> "/dev/tcp/$1/3306") 2> /dev/null || return 0
        tmp=$(mktemp "$DUMP_DIR/.tmp.$1.XXXXXX")
        # reachable: a dump failure is a real error (bad creds, etc.), fail, do not skip silently.
        # --skip-ssl: the alpine mariadb client defaults to TLS over TCP, which the mariadb server
        # image does not offer (error 2026), so disable it on this in-network connection.
        # sed: strip the sandbox-mode preamble (`/*M!999999\- ... */`) newer mariadb-dump emits; a
        # restore client that does not support it aborts with "Unknown command '\-'". pipefail keeps
        # a mariadb-dump failure fatal through the pipe.
        if ! MYSQL_PWD="$p" mariadb-dump --skip-ssl -h "$1" -u "$u" --single-transaction "$d" \
                | sed '/enable the sandbox mode/d' > "$tmp"; then
            echo "mariadb-dump failed for $1 (reachable but dump errored)" >&2; rm -f "$tmp"; return 1
        fi
        [ -s "$tmp" ] || { echo "empty mariadb dump for $1" >&2; rm -f "$tmp"; return 1; }
        mv "$tmp" "$DUMP_DIR/$1.sql"
    elif [ -n "$2" ]; then
        # Strip the sandbox-mode preamble too, so host-made dumps restore on any client (e.g. the
        # documented docker <-> bare metal cross restore).
        mariadb-dump --single-transaction "$2" | sed '/enable the sandbox mode/d' > "$DUMP_DIR/$1.sql"
    fi
}

stage_configs() {
    local f
    mkdir -p "$DUMP_DIR/config"
    for f in $CONFIG_FILES; do
        if [ -f "$f" ]; then cp -p --parents "$f" "$DUMP_DIR/config/"; fi
    done
}

snapshot() {
    local targets
    mapfile -t targets < <(data_targets)
    restic cat config > /dev/null 2>&1 || restic init
    restic backup --host arcadia --tag arcadia "$DUMP_DIR"/*.sql "$DUMP_DIR/config" "${targets[@]}"
    # shellcheck disable=SC2046 # keep holds several flags
    restic forget --host arcadia --prune $(cfg keep)
}

push() {
    local remote
    for remote in $REMOTES; do
        restic -r "$remote" cat config > /dev/null 2>&1 ||
            restic -r "$remote" init --from-repo "$REPO" --copy-chunker-params
        restic -r "$remote" copy --from-repo "$REPO"
        # shellcheck disable=SC2046 # keep holds several flags
        restic -r "$remote" forget --host arcadia --prune $(cfg keep)
    done
}

mkdir -p "$DUMP_DIR" "$REPO"
umask 077
check_current_state
clean_dumps
trap clean_dumps EXIT
dump_postgres
dump_mariadb ergo_database "$(cfg ergo_db)" ERGO_DB
dump_mariadb chevereto_database "$(cfg chevereto_db)" CHEVERETO_DB
stage_configs
snapshot
push
