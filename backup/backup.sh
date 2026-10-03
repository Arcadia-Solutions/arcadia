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
    if [ "$MODE" = docker ]; then
        docker compose exec -T db pg_dump -U "$user" -d "$name" --no-owner --no-privileges
    else
        PGPASSWORD=$(config_value database password) pg_dump -h "$(config_value database host)" \
            -p "$(config_value database port)" -U "$user" -d "$name" --no-owner --no-privileges
    fi > "$DUMP_DIR/arcadia.sql"
}

# $1: compose service, also the dump name. $2: database name in host mode. Skipped when the service
# is not running (docker) or $2 is empty (host): an empty dump would wipe the database on restore.
dump_mariadb() {
    if [ "$MODE" = docker ]; then
        [ -n "$(docker compose ps -q --status running "$1" 2> /dev/null)" ] || return 0
        docker compose exec -T "$1" sh -c \
            'MYSQL_PWD="$MYSQL_ROOT_PASSWORD" exec mariadb-dump -uroot --single-transaction "$MYSQL_DATABASE"' \
            > "$DUMP_DIR/$1.sql"
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
    local targets v
    mapfile -t targets < <(data_targets)
    # docker run would silently create a missing volume and back it up empty
    if [ "$MODE" = docker ]; then for v in $VOLUMES; do docker volume inspect "$v" > /dev/null; done; fi
    restic cat config > /dev/null 2>&1 || restic init
    restic backup --host arcadia --tag arcadia "$DUMP_DIR"/*.sql "$DUMP_DIR/config" "${targets[@]}"
    # shellcheck disable=SC2046 # keep holds several flags
    restic forget --host arcadia --prune $(cfg keep)
}

push() {
    [ -n "$REMOTE" ] || return 0
    restic -r "$REMOTE" cat config > /dev/null 2>&1 ||
        restic -r "$REMOTE" init --from-repo "$REPO" --copy-chunker-params
    restic -r "$REMOTE" copy --from-repo "$REPO"
    # shellcheck disable=SC2046
    restic -r "$REMOTE" forget --host arcadia --prune $(cfg keep)
}

mkdir -p "$DUMP_DIR" "$REPO"
umask 077
clean_dumps
trap clean_dumps EXIT
dump_postgres
dump_mariadb ergo_database "$(cfg ergo_db)"
dump_mariadb chevereto_database "$(cfg chevereto_db)"
stage_configs
snapshot
push
