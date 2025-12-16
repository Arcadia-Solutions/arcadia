# Backup

This page explains how to create backups of your Arcadia installation, including the database, configuration files, and frontend assets.

## Quick Start

### Basic Backup (Docker)
```bash
./backup.sh --db-docker
```

### Encrypted Backup (Recommended)
```bash
./backup.sh --db-docker --encrypt --password "your_secure_password"
```

### Full Help
```bash
./backup.sh --help
```

## What Gets Backed Up

| Content | Location in Backup | Description |
|---------|-------------------|-------------|
| Database | `database_full.sql` | Complete PostgreSQL dump (schema + data) |
| Environment Files | `env_files/` | All `.env` files from the project |
| Frontend Assets | `frontend_public/` | Contents of `frontend/public/home/` |
| Backup Info | `backup_info.txt` | Metadata and timestamps |

## Encryption

Backups can be encrypted using GPG with AES256 symmetric encryption.

### Enable Encryption
```bash
./backup.sh --db-docker --encrypt --password "your_secure_password"
```

### Using a Password File (More Secure)
```bash
echo "your_secure_password" > /path/to/password_file
chmod 600 /path/to/password_file
./backup.sh --db-docker --encrypt --password-file /path/to/password_file
```

### Output Format
- **Without encryption:** `arcadia_backup_YYYYMMDD_HHMMSS.zip`
- **With encryption:** `arcadia_backup_YYYYMMDD_HHMMSS.tar.gz.gpg`

### Decrypting a Backup
```bash
gpg --decrypt --passphrase "your_password" --batch backup.tar.gz.gpg | tar -xzf -
```

## Custom Destination

Save backups to a specific directory:
```bash
./backup.sh --db-docker --destination /path/to/backups
```

## Remote Upload

Upload backups to a remote server automatically.

### Using rsync
```bash
./backup.sh --db-docker --encrypt --password "secret" \
    --remote-type rsync \
    --remote-host backup.example.com \
    --remote-user backup \
    --remote-path /backups/arcadia
```

### Using scp
```bash
./backup.sh --db-docker --encrypt --password "secret" \
    --remote-type scp \
    --remote-host backup.example.com \
    --remote-user backup \
    --remote-path /backups/arcadia
```

### Additional Options
- `--remote-key /path/to/key` - Use a specific SSH key
- `--remote-port 2222` - Use a non-standard SSH port
- `--delete-after-upload` - Delete local backup after successful upload

## Scheduled Backups

### Option 1: Docker Daemon (Recommended)

The backup container can run in daemon mode with its own internal cron scheduler. This is the easiest setup - no host cron configuration needed.

1. **Create configuration file:**
   ```bash
   cp backup.conf.example backup.conf
   nano backup.conf  # Edit with your settings
   ```

2. **Start the backup daemon:**
   ```bash
   BACKUP_MODE=daemon docker compose --profile backup up -d backup
   ```

3. **Configure the schedule (optional):**

   Set `BACKUP_CRON_SCHEDULE` when starting:
   ```bash
   BACKUP_MODE=daemon BACKUP_CRON_SCHEDULE="0 */6 * * *" docker compose --profile backup up -d backup
   ```

   Schedule examples:
   - `0 3 * * *` - Daily at 3:00 AM (default)
   - `0 */6 * * *` - Every 6 hours
   - `0 2 * * 0` - Weekly on Sunday at 2:00 AM

4. **View logs:**
   ```bash
   docker logs -f arcadia_backup
   ```

5. **Stop the daemon:**
   ```bash
   docker compose --profile backup down
   ```

### Option 2: System Cron

If you prefer to use your host system's cron instead of the Docker daemon:

1. **Create configuration file:**
   ```bash
   cp backup.conf.example backup.conf
   nano backup.conf  # Edit with your settings
   ```

2. **Make scripts executable:**
   ```bash
   chmod +x backup.sh backup-cron.sh
   ```

3. **Add to crontab:**
   ```bash
   # Daily at 3:00 AM
   crontab -e
   # Add: 0 3 * * * /path/to/arcadia/backup-cron.sh
   ```

### Option 3: Docker Oneshot via Cron

Run backup via Docker triggered by host cron:
```bash
docker compose --profile backup run --rm backup
```

Add to crontab for scheduled Docker backups:
```bash
0 3 * * * cd /path/to/arcadia && docker compose --profile backup run --rm backup
```

## Configuration File

The `backup.conf` file configures scheduled backups. Copy from example:

```bash
cp backup.conf.example backup.conf
```

### Configuration Options

```bash
# Encryption
BACKUP_ENCRYPT=true
BACKUP_PASSWORD=your_secure_passphrase

# Local storage
BACKUP_DESTINATION=/home/ubuntu/arcadia-backups

# Remote storage (optional)
BACKUP_REMOTE_TYPE=rsync
BACKUP_REMOTE_HOST=backup.example.com
BACKUP_REMOTE_USER=backup
BACKUP_REMOTE_PATH=/backups/arcadia

# Retention policy
BACKUP_RETENTION_DAYS=7

# Logging
BACKUP_LOG_FILE=/var/log/arcadia-backup.log
```

## Retention Policy

Old backups are automatically deleted based on `BACKUP_RETENTION_DAYS`:

```bash
# In backup.conf
BACKUP_RETENTION_DAYS=7  # Keep backups for 7 days
```

The cron wrapper (`backup-cron.sh`) handles this automatically.

## Restore Procedure

### 1. Decrypt the Backup (if encrypted)
```bash
gpg --decrypt --passphrase "your_password" --batch \
    arcadia_backup_YYYYMMDD_HHMMSS.tar.gz.gpg | tar -xzf -
```

### 2. Restore Database
```bash
# Docker setup
docker exec -i arcadia_db psql -U arcadia -d arcadia < backup_*/database_full.sql

# Local PostgreSQL
psql -h localhost -U arcadia -d arcadia < backup_*/database_full.sql
```

### 3. Restore Environment Files
```bash
# Copy .env files back to their locations
cp backup_*/env_files/backend/api/.env backend/api/
cp backup_*/env_files/frontend/.env frontend/
cp backup_*/env_files/tracker/arcadia_tracker/.env tracker/arcadia_tracker/
```

### 4. Restore Frontend Assets
```bash
cp -r backup_*/frontend_public/home/* frontend/public/home/
```

## Command Reference

```
Usage: ./backup.sh [OPTIONS]

DATABASE OPTIONS:
  --db-docker          Use Docker setup
  --db-container NAME  Docker container name (default: arcadia_db)
  --db-host HOST       Database host (default: localhost)
  --db-port PORT       Database port (default: 5432)
  --db-name NAME       Database name (default: arcadia)
  --db-user USER       Database user (default: arcadia)
  --db-password PASS   Database password

ENCRYPTION OPTIONS:
  --encrypt            Enable GPG encryption (AES256)
  --password PASS      Encryption password
  --password-file FILE Read password from file

DESTINATION OPTIONS:
  --destination DIR    Output directory
  --keep-temp          Keep temporary backup directory

REMOTE UPLOAD OPTIONS:
  --remote-type TYPE   Upload method: rsync or scp
  --remote-host HOST   Remote server hostname
  --remote-user USER   Remote server username
  --remote-path PATH   Remote destination path
  --remote-key FILE    SSH private key file
  --remote-port PORT   SSH port (default: 22)
  --delete-after-upload Delete local backup after upload

GENERAL OPTIONS:
  -h, --help           Show help message
```

## Troubleshooting

### "Database container not running"
Start the database container:
```bash
docker compose up -d db
```

### "gpg: decryption failed"
- Verify you're using the correct password
- Check the file isn't corrupted: `file backup.tar.gz.gpg`

### "Permission denied" on cron
- Ensure scripts are executable: `chmod +x backup.sh backup-cron.sh`
- Check log file permissions: `touch /var/log/arcadia-backup.log && chmod 666 /var/log/arcadia-backup.log`

### Remote upload fails
- Test SSH connection: `ssh user@host`
- Check SSH key permissions: `chmod 600 /path/to/key`
- Verify remote path exists: `ssh user@host "mkdir -p /backups/arcadia"`

## Logs

### Docker Daemon Mode
When using the backup daemon, logs are available via Docker:
```bash
docker logs -f arcadia_backup
```

### System Cron Mode
When using `backup-cron.sh`, logs are written to:
```
/var/log/arcadia-backup.log
```

View recent logs:
```bash
tail -50 /var/log/arcadia-backup.log
```
