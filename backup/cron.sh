#!/bin/sh
# Runs in the backup_cron service (compose profile `backup`): turns backup.cron of config.yml into the
# root crontab and hands over to cron. Docker mode only: the job is backup/backup.sh, run in this
# container (dumps over the compose network, restic in-container, no docker socket).
set -eu
cd "$(dirname "$0")/.." || exit 1
. scripts/config_value.sh
cfg() { config_value backup "$1"; }

CRON=$(cfg cron)
[ -n "$CRON" ] || { echo "backup.cron is empty, there is nothing to schedule" >&2; exit 1; }
# Containers have no timezone, cron needs one to know when the schedule fires.
TZ=$(cfg cron_timezone)
export TZ="${TZ:-UTC}"
mkdir -p /etc/crontabs
printf '%s /arcadia/backup/backup.sh >>/proc/1/fd/1 2>&1 || echo "backup.sh failed: $?" >>/proc/1/fd/1\n' \
    "$CRON" > /etc/crontabs/root
chmod 600 /etc/crontabs/root
echo "scheduled in backup.cron: $(cat /etc/crontabs/root)"
exec crond -f -c /etc/crontabs
