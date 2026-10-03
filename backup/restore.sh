#!/usr/bin/env bash
# Restores a snapshot made by backup.sh: configuration files, volumes (docker) or data directories
# (host) and databases. Usage: backup/restore.sh [snapshot] [-y], the snapshot defaults to latest.
# Configured by the `backup` section of config.yml, see docs/src/backup.md.
cd "$(dirname "$0")/.." || exit 1
. backup/lib.sh

SNAP=latest
YES=
SERVICES=
for arg in "$@"; do
    case $arg in -y) YES=1 ;; *) SNAP=$arg ;; esac
done

confirm() {
    restic snapshots "$SNAP"
    if [ -n "$YES" ]; then return 0; fi
    local answer
    read -rp "Overwrite the databases, data and configuration files with snapshot $SNAP? Type yes: " answer
    [ "$answer" = yes ] || exit 1
}

# Docker: stops every running service, profiles included, to start them again at the end (redis and
# ergo would overwrite restored files on shutdown). Host: the services are not ours to manage.
stop_services() {
    if [ "$MODE" = docker ]; then
        SERVICES=$(docker compose ps --services --status running)
        if [ -n "$SERVICES" ]; then docker compose stop $SERVICES; fi
    else
        local host port
        host=$(config_value api host)
        port=$(config_value api port)
        if curl -sf -o /dev/null "http://${host:-127.0.0.1}:${port:-8080}/health"; then
            echo "the backend answers: stop arcadia, ergo and redis before restoring" >&2
            exit 1
        fi
    fi
}

# Host mode drops and creates the cluster as the postgres superuser, which the database section
# holds no password for: it goes through the unix socket, which only reaches the local cluster. The
# configured port picks the socket, so dropdb, createdb and psql all land on the same cluster.
# Checked before anything is overwritten.
check_postgres() {
    if [ "$MODE" = docker ]; then return 0; fi
    local host
    host=$(config_value database host)
    case $host in
        "" | localhost | 127.0.0.1) ;;
        *) echo "host mode restores the local postgres cluster, but database.host is '$host'" >&2; exit 1 ;;
    esac
}

# The dumps and configuration files. restic runs as root and restores them root owned: the files it
# overwrites keep the owner they had, the new ones get the owner of the repository, so the checkout
# stays usable without sudo. The database credentials are read from the restored config.yml from
# here on.
restore_files() {
    local f dest owner
    restic restore "$SNAP:$DUMP_DIR" --target "$DUMP_DIR"
    # staged with --parents, so ergo/ and kiwiirc/ are directories of configuration files: they are
    # restored file by file, the checkout holds more in them than the snapshot (the examples), and
    # find leaves the loop empty when the snapshot holds no configuration file at all.
    while IFS= read -r f; do
        dest=${f#"$DUMP_DIR/config/"}
        owner=$(stat -c %u:%g "$dest" 2> /dev/null || stat -c %u:%g .)
        mkdir -p "$(dirname "$dest")"
        cp -a "$f" "$dest"
        chown "$owner" "$dest"
    done < <(find "$DUMP_DIR/config" -type f 2> /dev/null)
}

restore_data() {
    local t
    for t in $(data_targets); do restic restore "$SNAP:$t" --target "$t" --delete; done
}

restore_postgres() {
    local user name port
    user=$(config_value database user)
    name=$(config_value database name)
    if [ "$MODE" = docker ]; then
        docker compose up -d --wait db
        docker compose exec -T db dropdb -U "$user" --if-exists --force "$name"
        docker compose exec -T db createdb -U "$user" "$name"
        docker compose exec -T db psql -q -v ON_ERROR_STOP=1 -U "$user" -d "$name" < "$DUMP_DIR/arcadia.sql"
    else
        port=$(config_value database port)
        runuser -u postgres -- dropdb -p "$port" --if-exists --force "$name"
        runuser -u postgres -- createdb -p "$port" -O "$user" "$name"
        PGPASSWORD=$(config_value database password) psql -q -v ON_ERROR_STOP=1 \
            -h "$(config_value database host)" -p "$port" -U "$user" -d "$name" \
            < "$DUMP_DIR/arcadia.sql"
    fi
}

# mariadb as root, in compose service $1 (docker) or on the host
mariadb_cli() {
    local service=$1
    shift
    if [ "$MODE" = docker ]; then
        docker compose exec -T "$service" sh -c 'MYSQL_PWD="$MYSQL_ROOT_PASSWORD" exec mariadb -uroot "$@"' - "$@"
    else
        mariadb "$@"
    fi
}

# $1: compose service, also the dump name. $2: database name in host mode. Skipped without a dump.
restore_mariadb() {
    [ -f "$DUMP_DIR/$1.sql" ] || return 0
    local db=$2
    if [ "$MODE" = docker ]; then
        docker compose up -d --wait "$1"
        db=$(docker compose exec -T "$1" printenv MYSQL_DATABASE)
    fi
    mariadb_cli "$1" -e "DROP DATABASE IF EXISTS \`$db\`; CREATE DATABASE \`$db\`"
    mariadb_cli "$1" "$db" < "$DUMP_DIR/$1.sql"
}

mkdir -p "$DUMP_DIR"
confirm
clean_dumps
trap clean_dumps EXIT
stop_services
check_postgres
restore_files
restore_data
restore_postgres
restore_mariadb ergo_database "$(cfg ergo_db)"
restore_mariadb chevereto_database "$(cfg chevereto_db)"
if [ -n "$SERVICES" ]; then docker compose up -d $SERVICES; fi
