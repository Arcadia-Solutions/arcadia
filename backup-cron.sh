#!/bin/bash

# Arcadia Backup Cron Wrapper
# This script is designed to be run by cron for scheduled backups
# It loads configuration from backup.conf and handles logging/locking
#
# Installation:
#   1. Copy backup.conf.example to backup.conf and customize
#   2. chmod +x backup-cron.sh
#   3. Add to crontab: 0 3 * * * /path/to/backup-cron.sh
#
# Logs are written to /var/log/arcadia-backup.log by default

set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Default configuration
LOG_FILE="${BACKUP_LOG_FILE:-/var/log/arcadia-backup.log}"
LOCK_FILE="/tmp/arcadia-backup.lock"

# ============================================================================
# LOGGING
# ============================================================================

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" >> "$LOG_FILE"
}

log_separator() {
    echo "========================================" >> "$LOG_FILE"
}

# ============================================================================
# LOCK HANDLING
# ============================================================================

# Prevent concurrent runs
if [ -f "$LOCK_FILE" ]; then
    # Check if the process is still running
    if [ -f "$LOCK_FILE" ]; then
        OLD_PID=$(cat "$LOCK_FILE" 2>/dev/null)
        if [ -n "$OLD_PID" ] && kill -0 "$OLD_PID" 2>/dev/null; then
            log "ERROR: Backup already running (PID: $OLD_PID), skipping"
            exit 0
        else
            log "WARNING: Stale lock file found, removing"
            rm -f "$LOCK_FILE"
        fi
    fi
fi

# Create lock file with our PID
echo $$ > "$LOCK_FILE"
trap "rm -f $LOCK_FILE" EXIT

# ============================================================================
# LOAD CONFIGURATION
# ============================================================================

if [ -f "$SCRIPT_DIR/backup.conf" ]; then
    source "$SCRIPT_DIR/backup.conf"
    log "Configuration loaded from backup.conf"
else
    log "WARNING: No backup.conf found, using defaults"
fi

# ============================================================================
# RUN BACKUP
# ============================================================================

log_separator
log "Starting scheduled backup"

cd "$SCRIPT_DIR"

# Build command arguments
BACKUP_ARGS="--db-docker"

[ "$BACKUP_ENCRYPT" = "true" ] && BACKUP_ARGS="$BACKUP_ARGS --encrypt"
[ -n "$BACKUP_PASSWORD" ] && BACKUP_ARGS="$BACKUP_ARGS --password \"$BACKUP_PASSWORD\""
[ -n "$BACKUP_PASSWORD_FILE" ] && BACKUP_ARGS="$BACKUP_ARGS --password-file \"$BACKUP_PASSWORD_FILE\""
[ -n "$BACKUP_DESTINATION" ] && BACKUP_ARGS="$BACKUP_ARGS --destination \"$BACKUP_DESTINATION\""
[ -n "$BACKUP_REMOTE_TYPE" ] && BACKUP_ARGS="$BACKUP_ARGS --remote-type \"$BACKUP_REMOTE_TYPE\""
[ -n "$BACKUP_REMOTE_HOST" ] && BACKUP_ARGS="$BACKUP_ARGS --remote-host \"$BACKUP_REMOTE_HOST\""
[ -n "$BACKUP_REMOTE_USER" ] && BACKUP_ARGS="$BACKUP_ARGS --remote-user \"$BACKUP_REMOTE_USER\""
[ -n "$BACKUP_REMOTE_PATH" ] && BACKUP_ARGS="$BACKUP_ARGS --remote-path \"$BACKUP_REMOTE_PATH\""
[ -n "$BACKUP_REMOTE_KEY" ] && BACKUP_ARGS="$BACKUP_ARGS --remote-key \"$BACKUP_REMOTE_KEY\""
[ -n "$BACKUP_REMOTE_PORT" ] && BACKUP_ARGS="$BACKUP_ARGS --remote-port \"$BACKUP_REMOTE_PORT\""
[ "$BACKUP_DELETE_AFTER_UPLOAD" = "true" ] && BACKUP_ARGS="$BACKUP_ARGS --delete-after-upload"

# Run backup
log "Executing: ./backup.sh $BACKUP_ARGS"
eval "./backup.sh $BACKUP_ARGS" >> "$LOG_FILE" 2>&1
STATUS=$?

if [ $STATUS -eq 0 ]; then
    log "Backup completed successfully"
else
    log "ERROR: Backup failed with exit code $STATUS"
fi

# ============================================================================
# RETENTION POLICY
# ============================================================================

if [ -n "$BACKUP_RETENTION_DAYS" ] && [ -n "$BACKUP_DESTINATION" ]; then
    log "Applying retention policy: deleting backups older than $BACKUP_RETENTION_DAYS days"

    # Count files before deletion
    OLD_COUNT=$(find "$BACKUP_DESTINATION" -name "arcadia_backup_*" -mtime +$BACKUP_RETENTION_DAYS 2>/dev/null | wc -l)

    if [ "$OLD_COUNT" -gt 0 ]; then
        find "$BACKUP_DESTINATION" -name "arcadia_backup_*" -mtime +$BACKUP_RETENTION_DAYS -delete
        log "Deleted $OLD_COUNT old backup(s)"
    else
        log "No old backups to delete"
    fi
fi

# ============================================================================
# SUMMARY
# ============================================================================

if [ -n "$BACKUP_DESTINATION" ] && [ -d "$BACKUP_DESTINATION" ]; then
    BACKUP_COUNT=$(find "$BACKUP_DESTINATION" -name "arcadia_backup_*" 2>/dev/null | wc -l)
    BACKUP_SIZE=$(du -sh "$BACKUP_DESTINATION" 2>/dev/null | cut -f1)
    log "Current backups: $BACKUP_COUNT files, total size: $BACKUP_SIZE"
fi

log "Scheduled backup job finished"
log_separator

exit $STATUS
