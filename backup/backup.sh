#!/usr/bin/env bash
# Dumps the databases, then snapshots the dumps, the configuration files and the volumes (docker) or
# data directories (host) into the local restic repository, prunes it and copies the snapshots to
# backup.remote_repo when set. Configured by the `backup` section of config.yml, see
# docs/src/backup.md.
cd "$(dirname "$0")/.." || exit 1
. backup/lib.sh

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
        if ! MYSQL_PWD="$p" mariadb-dump -h "$1" -u "$u" --single-transaction "$d" > "$tmp"; then
            echo "mariadb-dump failed for $1 (reachable but dump errored)" >&2; rm -f "$tmp"; return 1
        fi
        [ -s "$tmp" ] || { echo "empty mariadb dump for $1" >&2; rm -f "$tmp"; return 1; }
        mv "$tmp" "$DUMP_DIR/$1.sql"
    elif [ -n "$2" ]; then
        mariadb-dump --single-transaction "$2" > "$DUMP_DIR/$1.sql"
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
clean_dumps
trap clean_dumps EXIT
dump_postgres
dump_mariadb ergo_database "$(cfg ergo_db)" ERGO_DB
dump_mariadb chevereto_database "$(cfg chevereto_db)" CHEVERETO_DB
stage_configs
snapshot
push
