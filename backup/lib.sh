# Shared by backup.sh and restore.sh, sourced from the repository root. Reads the `backup` section
# of the configuration file, documented in config.example.yml.
set -euo pipefail
. scripts/config_value.sh

cfg() { config_value backup "$1"; }
MODE=$(cfg mode)
REPO=$(cfg repo)
PASSWORD_FILE=$(cfg password_file)
DUMP_DIR=$(cfg dump_dir)
VOLUMES=$(cfg volumes)
PATHS=$(cfg paths)
REMOTE=$(cfg remote_repo)
RESTIC_IMAGE=restic/restic:0.19.1
# The gitignored configuration files of the repository, skipped when absent.
CONFIG_FILES="config.yml ergo/ergo-conf.yaml ergo/ergo.motd kiwiirc/config.json compose.override.yml"

case $MODE in docker | host) ;; *) echo "backup.mode must be docker or host, not '$MODE'" >&2; exit 1 ;; esac
for p in "$REPO" "$DUMP_DIR"; do
    case $p in /?*) ;; *) echo "backup.repo and backup.dump_dir must be absolute paths" >&2; exit 1 ;; esac
done
case $REMOTE in "" | /* | *:*) ;; *) echo "backup.remote_repo must be absolute or a restic backend URL" >&2; exit 1 ;; esac
[ -r "$PASSWORD_FILE" ] || { echo "cannot read backup.password_file '$PASSWORD_FILE'" >&2; exit 1; }
# Without retention flags restic forget refuses to prune, which would abort every run after the
# snapshot and before the copy to remote_repo.
[ -n "$(cfg keep)" ] || { echo "backup.keep must hold restic forget flags, e.g. --keep-daily 7 --keep-weekly 4" >&2; exit 1; }
# Without a data target the snapshot holds the dumps and the configuration files only, which looks
# like a backup of an installation whose images and ircd are gone.
if [ "$MODE" = docker ]; then
    [ -n "$VOLUMES" ] || { echo "backup.volumes must list the volumes to back up in docker mode" >&2; exit 1; }
else
    [ -n "$PATHS" ] || { echo "backup.paths must list the data directories to back up in host mode" >&2; exit 1; }
fi

# What is backed up raw, as restic sees it: the volumes mounted by restic() or the host paths.
data_targets() {
    local t
    if [ "$MODE" = docker ]; then for t in $VOLUMES; do echo "/data/$t"; done
    else for t in $PATHS; do echo "$t"; done; fi
}

# restic on the local repository, from the pinned image (docker) or the host. dump_dir and the
# repositories are mounted at their own path so that snapshots look the same in both modes.
restic() {
    if [ "$MODE" = docker ]; then
        local mounts=() v
        for v in $VOLUMES; do mounts+=(-v "$v:/data/$v"); done
        case $REMOTE in /*) mounts+=(-v "$REMOTE:$REMOTE") ;; esac
        docker run --rm -e RESTIC_REPOSITORY="$REPO" -e RESTIC_PASSWORD_FILE=/password \
            -e RESTIC_FROM_PASSWORD_FILE=/password -e AWS_ACCESS_KEY_ID -e AWS_SECRET_ACCESS_KEY \
            -v "$PASSWORD_FILE:/password:ro" -v "$REPO:$REPO" -v "$DUMP_DIR:$DUMP_DIR" "${mounts[@]}" \
            "$RESTIC_IMAGE" --retry-lock 30m "$@"
    else
        RESTIC_REPOSITORY="$REPO" RESTIC_PASSWORD_FILE="$PASSWORD_FILE" \
            RESTIC_FROM_PASSWORD_FILE="$PASSWORD_FILE" command restic --retry-lock 30m "$@"
    fi
}

# Only what the scripts write: dump_dir may hold other files.
clean_dumps() { rm -rf "$DUMP_DIR"/{arcadia,ergo_database,chevereto_database}.sql "$DUMP_DIR/config"; }
