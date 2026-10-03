#!/usr/bin/env bash
# Runs on the backup host: copies the snapshots of arcadia's repository missing from the local one
# over sftp, fails when none is new, then checks and prunes the local repository. Configured by the
# `backup_pull` section of the file ARCADIA_CONFIG points at, never by arcadia's config.yml: values
# coming from arcadia must not end up in commands run here. See docs/src/backup.md.
cd "$(dirname "$0")/.." || exit 1
set -euo pipefail
. scripts/config_value.sh

cfg() { config_value backup_pull "$1"; }
SSH=$(cfg ssh)
SOURCE=sftp:$SSH:$(cfg source_repo)
TRIGGER=$(cfg trigger)
RESTIC_REPOSITORY=$(cfg repo)
RESTIC_PASSWORD_FILE=$(cfg password_file)
export RESTIC_REPOSITORY RESTIC_PASSWORD_FILE RESTIC_FROM_PASSWORD_FILE=$RESTIC_PASSWORD_FILE

restic() { command restic --retry-lock 30m "$@"; }

if [ -n "$TRIGGER" ]; then ssh "$SSH" "$TRIGGER"; fi
restic cat config > /dev/null 2>&1 || restic init --from-repo "$SOURCE" --copy-chunker-params
before=$(restic list snapshots | sort)
restic copy --from-repo "$SOURCE"
if [ "$before" = "$(restic list snapshots | sort)" ]; then echo "no new snapshot in $SOURCE" >&2; exit 1; fi
restic check --read-data-subset 5%
# shellcheck disable=SC2046 # keep holds several flags
restic forget --host arcadia --prune $(cfg keep)
