#!/bin/bash

# Arcadia Backup Entrypoint
# Supports two modes:
#   - oneshot: Run backup once and exit (default)
#   - daemon: Run crond in foreground with scheduled backups

set -e

BACKUP_MODE="${BACKUP_MODE:-oneshot}"
BACKUP_CRON_SCHEDULE="${BACKUP_CRON_SCHEDULE:-0 3 * * *}"

# ============================================================================
# DAEMON MODE
# ============================================================================

if [ "$BACKUP_MODE" = "daemon" ]; then
    echo "============================================"
    echo "Arcadia Backup Daemon"
    echo "============================================"
    echo "Mode: daemon"
    echo "Schedule: $BACKUP_CRON_SCHEDULE"
    echo "Timezone: ${TZ:-UTC}"
    echo ""

    # Create log file
    touch /var/log/arcadia-backup.log

    # Create crontab entry
    # The cron job runs backup-cron.sh which handles locking, logging, and retention
    echo "$BACKUP_CRON_SCHEDULE /app/backup-cron.sh >> /var/log/arcadia-backup.log 2>&1" > /etc/crontabs/root

    echo "Crontab configured:"
    cat /etc/crontabs/root
    echo ""
    echo "Backup daemon started. Logs: /var/log/arcadia-backup.log"
    echo "To view logs: docker logs -f arcadia_backup_daemon"
    echo "============================================"
    echo ""

    # Tail the log file in background so docker logs shows backup output
    tail -F /var/log/arcadia-backup.log 2>/dev/null &

    # Run crond in foreground
    exec crond -f -l 2

# ============================================================================
# ONESHOT MODE (default)
# ============================================================================

else
    echo "Mode: oneshot"
    exec /app/backup.sh "$@"
fi
