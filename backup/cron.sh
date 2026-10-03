#!/bin/sh
# Runs in the backup_cron service (compose profile `backup`): turns backup.cron of config.yml into the
# root crontab and hands over to cron. Docker mode only: the job is backup/backup.sh, which drives
# docker compose and runs restic from its image.
set -eu
cd "$(dirname "$0")/.." || exit 1
. scripts/config_value.sh
cfg() { config_value backup "$1"; }

CRON=$(cfg cron)
[ -n "$CRON" ] || { echo "backup.cron is empty, there is nothing to schedule" >&2; exit 1; }
# Containers have no timezone, cron needs one to know when the schedule fires.
TZ=$(cfg cron_timezone)
export TZ="${TZ:-UTC}"
# What compose labelled this container with: the project of the installation, which the job has to act
# on, and the checkout on the host, mounted at its own path so that compose names the project the same
# way from inside the run.
PROJECT=$(docker inspect --format '{{index .Config.Labels "com.docker.compose.project"}}' "$(hostname)")
REPO=$(docker inspect --format '{{index .Config.Labels "com.docker.compose.project.working_dir"}}' "$(hostname)")
IMAGE=$(docker inspect --format '{{.Config.Image}}' "$(hostname)")
DUMP_DIR=$(cfg dump_dir)
PASSWORD_FILE=$(cfg password_file)
# Each run gets a container of its own, holding what backup.sh needs at the paths config.yml gives
# them: compose cannot mount them, it resolves the volumes of a service before this container exists.
#
# busybox crond runs the job from / in a minimal environment, drops its output and ignores its exit
# code: keep both in the container log instead. It also refuses a crontab that is not mode 600.
mkdir -p /etc/crontabs
printf '%s docker run --rm --entrypoint %s/backup/backup.sh -e TZ=%s -e COMPOSE_PROJECT_NAME=%s -v %s:%s:ro -v /var/run/docker.sock:/var/run/docker.sock -v %s:%s -v %s:%s:ro %s >>/proc/1/fd/1 2>&1 || echo "backup.sh failed: $?" >>/proc/1/fd/1\n' \
    "$CRON" "$REPO" "$TZ" "$PROJECT" "$REPO" "$REPO" "$DUMP_DIR" "$DUMP_DIR" "$PASSWORD_FILE" \
    "$PASSWORD_FILE" "$IMAGE" > /etc/crontabs/root
chmod 600 /etc/crontabs/root
echo "scheduled in backup.cron: $(cat /etc/crontabs/root)"
exec crond -f -c /etc/crontabs
