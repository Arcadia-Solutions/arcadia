#!/bin/bash

# Arcadia Backup Script
# This script backs up the database, environment files, and frontend assets
# Supports Docker and non-Docker setups, encryption, and remote upload
#
# Usage: ./backup.sh [OPTIONS]
# Run ./backup.sh --help for full options

set -e  # Exit on any error

# ============================================================================
# DEFAULT VALUES
# ============================================================================

USE_DOCKER=false
ENCRYPT=false
BACKUP_PASSWORD=""
BACKUP_PASSWORD_FILE=""
BACKUP_DESTINATION="${BACKUP_DESTINATION:-.}"
KEEP_TEMP=false
REMOTE_ENABLED=false
REMOTE_TYPE=""
REMOTE_HOST=""
REMOTE_USER=""
REMOTE_PATH=""
REMOTE_KEY=""
REMOTE_PORT="22"
DELETE_AFTER_UPLOAD=false

# Database defaults
DB_NAME=""
DB_USER=""
DB_PASSWORD=""
DB_HOST=""
DB_PORT=""
DB_CONTAINER=""

# ============================================================================
# PARSE COMMAND LINE ARGUMENTS
# ============================================================================

while [[ $# -gt 0 ]]; do
    case $1 in
        # Database options
        --db-docker)
            USE_DOCKER=true
            shift
            ;;
        --db-name)
            DB_NAME="$2"
            shift 2
            ;;
        --db-user)
            DB_USER="$2"
            shift 2
            ;;
        --db-container)
            DB_CONTAINER="$2"
            shift 2
            ;;
        --db-password)
            DB_PASSWORD="$2"
            shift 2
            ;;
        --db-host)
            DB_HOST="$2"
            shift 2
            ;;
        --db-port)
            DB_PORT="$2"
            shift 2
            ;;
        # Encryption options
        --encrypt)
            ENCRYPT=true
            shift
            ;;
        --password)
            BACKUP_PASSWORD="$2"
            shift 2
            ;;
        --password-file)
            BACKUP_PASSWORD_FILE="$2"
            shift 2
            ;;
        # Destination options
        --destination)
            BACKUP_DESTINATION="$2"
            shift 2
            ;;
        --keep-temp)
            KEEP_TEMP=true
            shift
            ;;
        # Remote upload options
        --remote-type)
            REMOTE_ENABLED=true
            REMOTE_TYPE="$2"
            shift 2
            ;;
        --remote-host)
            REMOTE_HOST="$2"
            shift 2
            ;;
        --remote-user)
            REMOTE_USER="$2"
            shift 2
            ;;
        --remote-path)
            REMOTE_PATH="$2"
            shift 2
            ;;
        --remote-key)
            REMOTE_KEY="$2"
            shift 2
            ;;
        --remote-port)
            REMOTE_PORT="$2"
            shift 2
            ;;
        --delete-after-upload)
            DELETE_AFTER_UPLOAD=true
            shift
            ;;
        # Help
        -h|--help)
            cat << 'EOF'
Arcadia Backup Script - Creates a complete backup of the application

Usage: ./backup.sh [OPTIONS]

DATABASE OPTIONS:
  --db-docker          Use Docker setup (connects to database container)
  --db-container NAME  Docker container name (default: arcadia_db)
  --db-host HOST       Database host for local setup (default: localhost)
  --db-port PORT       Database port (default: 5432)
  --db-name NAME       Database name (default: arcadia)
  --db-user USER       Database user (default: arcadia)
  --db-password PASS   Database password

ENCRYPTION OPTIONS:
  --encrypt            Enable GPG encryption (AES256)
  --password PASS      Encryption password
  --password-file FILE Read encryption password from file

DESTINATION OPTIONS:
  --destination DIR    Output directory for backup (default: current dir)
  --keep-temp          Keep temporary backup directory

REMOTE UPLOAD OPTIONS:
  --remote-type TYPE   Upload method: rsync or scp
  --remote-host HOST   Remote server hostname
  --remote-user USER   Remote server username
  --remote-path PATH   Remote destination path
  --remote-key FILE    SSH private key file (optional)
  --remote-port PORT   SSH port (default: 22)
  --delete-after-upload Delete local backup after successful upload

GENERAL OPTIONS:
  -h, --help           Show this help message

EXAMPLES:
  ./backup.sh --db-docker
      Basic Docker backup

  ./backup.sh --db-docker --encrypt --password "secret123"
      Encrypted Docker backup

  ./backup.sh --db-docker --destination /backups --encrypt --password "secret"
      Encrypted backup to custom directory

  ./backup.sh --db-docker --encrypt --password "secret" \
      --remote-type rsync --remote-host backup.example.com \
      --remote-user backup --remote-path /backups/arcadia
      Encrypted backup with remote upload

ENVIRONMENT VARIABLES:
  BACKUP_PASSWORD      Default encryption password
  BACKUP_ENCRYPT       Set to 'true' to enable encryption by default

EOF
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# ============================================================================
# LOAD CONFIGURATION
# ============================================================================

BACKUP_DIR="backup_$(date +%Y%m%d_%H%M%S)"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Source configuration from backend .env files
if [ "$USE_DOCKER" = true ]; then
    if [ -f "backend/.env.docker" ]; then
        echo "Loading configuration from backend/.env.docker..."
        export $(grep -v '^#' backend/.env.docker | grep -E '^(POSTGRES_|DB_|BACKUP_)' | tr -d '\r' | xargs 2>/dev/null) || true
    elif [ -f "backend/.env" ]; then
        echo "Loading configuration from backend/.env..."
        export $(grep -v '^#' backend/.env | grep -E '^(POSTGRES_|DB_|BACKUP_)' | tr -d '\r' | xargs 2>/dev/null) || true
    fi
else
    if [ -f "backend/.env" ]; then
        echo "Loading configuration from backend/.env..."
        export $(grep -v '^#' backend/.env | grep -E '^(POSTGRES_|DB_|BACKUP_)' | tr -d '\r' | xargs 2>/dev/null) || true
    fi
fi

# Function to strip carriage returns
strip_cr() {
    echo "$1" | tr -d '\r'
}

# Apply defaults (command line > env vars > defaults)
[ -z "$DB_NAME" ] && DB_NAME=$(strip_cr "${POSTGRES_DATABASE:-arcadia}")
[ -z "$DB_USER" ] && DB_USER=$(strip_cr "${POSTGRES_USER:-arcadia}")
[ -z "$DB_PASSWORD" ] && DB_PASSWORD=$(strip_cr "${POSTGRES_PASSWORD}")
[ -z "$DB_HOST" ] && DB_HOST=$(strip_cr "${POSTGRES_HOST:-localhost}")
[ -z "$DB_PORT" ] && DB_PORT=$(strip_cr "${POSTGRES_PORT:-5432}")
[ -z "$DB_CONTAINER" ] && DB_CONTAINER=$(strip_cr "${DB_CONTAINER:-arcadia_db}")

# Check encryption env vars
[ "$BACKUP_ENCRYPT" = "true" ] && ENCRYPT=true
[ -z "$BACKUP_PASSWORD" ] && [ -n "${BACKUP_PASSWORD:-}" ] && BACKUP_PASSWORD="$BACKUP_PASSWORD"

# ============================================================================
# SETUP
# ============================================================================

echo "============================================"
echo "Arcadia Backup Script"
echo "============================================"
echo "Starting backup at $(date)"
echo ""

# Create temporary backup directory
mkdir -p "$BACKUP_DIR"

# Cleanup function
cleanup() {
    if [ "$KEEP_TEMP" = false ]; then
        echo "Cleaning up temporary files..."
        rm -rf "$BACKUP_DIR"
    else
        echo "Temporary files kept at: $BACKUP_DIR"
    fi
}
trap cleanup EXIT

# ============================================================================
# VALIDATE SETUP
# ============================================================================

if [ "$USE_DOCKER" = true ]; then
    echo "Mode: Docker"
    if ! docker info >/dev/null 2>&1; then
        echo "Error: Docker is not running or not accessible"
        exit 1
    fi
    if ! docker ps --format "table {{.Names}}" | grep -q "^$DB_CONTAINER$"; then
        echo "Error: Database container '$DB_CONTAINER' is not running"
        echo "Please start the database with: docker compose up db -d"
        exit 1
    fi
    echo "Database container: $DB_CONTAINER"
else
    echo "Mode: Local PostgreSQL"
    if ! command -v pg_dump >/dev/null 2>&1; then
        echo "Error: pg_dump command not found. Please install PostgreSQL client tools"
        exit 1
    fi
    echo "Database: $DB_HOST:$DB_PORT/$DB_NAME"
fi
echo ""

# ============================================================================
# BACKUP DATABASE
# ============================================================================

echo "Backing up database..."

if [ "$USE_DOCKER" = true ]; then
    docker exec "$DB_CONTAINER" pg_dump -U "$DB_USER" -d "$DB_NAME" --no-owner --no-privileges > "$BACKUP_DIR/database_full.sql"
    BACKUP_EXIT_CODE=$?
else
    if [ -n "$DB_PASSWORD" ]; then
        PGPASSWORD="$DB_PASSWORD" pg_dump \
            -h "$DB_HOST" \
            -p "$DB_PORT" \
            -U "$DB_USER" \
            -d "$DB_NAME" \
            --no-owner --no-privileges > "$BACKUP_DIR/database_full.sql"
    else
        pg_dump \
            -h "$DB_HOST" \
            -p "$DB_PORT" \
            -U "$DB_USER" \
            -d "$DB_NAME" \
            --no-owner --no-privileges > "$BACKUP_DIR/database_full.sql"
    fi
    BACKUP_EXIT_CODE=$?
fi

if [ $BACKUP_EXIT_CODE -eq 0 ]; then
    DB_SIZE=$(du -h "$BACKUP_DIR/database_full.sql" | cut -f1)
    echo "Database backup completed ($DB_SIZE)"
else
    echo "Error: Database backup failed"
    exit 1
fi

# ============================================================================
# BACKUP ALL .ENV FILES
# ============================================================================

echo ""
echo "Backing up environment files..."
mkdir -p "$BACKUP_DIR/env_files"

env_count=0
while IFS= read -r env_file; do
    if [ -n "$env_file" ]; then
        rel_path="${env_file#./}"
        target_dir="$BACKUP_DIR/env_files/$(dirname "$rel_path")"
        mkdir -p "$target_dir"
        cp "$env_file" "$target_dir/"
        echo "  Backed up: $rel_path"
        env_count=$((env_count + 1))
    fi
done < <(find . -name ".env*" -type f \
    ! -path "./node_modules/*" \
    ! -path "./.git/*" \
    ! -path "./target/*" \
    ! -path "./.sqlx/*" \
    ! -path "./backup_*/*" \
    2>/dev/null)

echo "Total .env files backed up: $env_count"

# ============================================================================
# BACKUP FRONTEND PUBLIC ASSETS
# ============================================================================

echo ""
echo "Backing up frontend public assets..."

if [ -d "frontend/public/home" ]; then
    mkdir -p "$BACKUP_DIR/frontend_public"
    cp -r "frontend/public/home" "$BACKUP_DIR/frontend_public/home"
    ASSETS_SIZE=$(du -sh "frontend/public/home" 2>/dev/null | cut -f1)
    echo "Frontend public/home backed up ($ASSETS_SIZE)"
else
    echo "Warning: frontend/public/home not found, skipping..."
fi

# ============================================================================
# CREATE BACKUP INFO FILE
# ============================================================================

cat > "$BACKUP_DIR/backup_info.txt" << EOF
Arcadia Backup
==============
Created on: $(date)
Timestamp: $TIMESTAMP

Database:
  Name: $DB_NAME
  Setup: $([ "$USE_DOCKER" = true ] && echo "Docker (container: $DB_CONTAINER)" || echo "Local PostgreSQL ($DB_HOST:$DB_PORT)")

Encryption: $([ "$ENCRYPT" = true ] && echo "Enabled (AES256)" || echo "Disabled")

Contents:
  - database_full.sql: Full database dump
  - env_files/: All environment configuration files
  - frontend_public/: Frontend public assets (if present)
  - backup_info.txt: This file
EOF

# ============================================================================
# CREATE ARCHIVE
# ============================================================================

echo ""
echo "Creating archive..."

if [ "$ENCRYPT" = true ]; then
    # Get password
    if [ -n "$BACKUP_PASSWORD_FILE" ] && [ -f "$BACKUP_PASSWORD_FILE" ]; then
        BACKUP_PASSWORD=$(cat "$BACKUP_PASSWORD_FILE")
    fi

    if [ -z "$BACKUP_PASSWORD" ]; then
        echo "Error: Encryption enabled but no password provided"
        echo "Use --password, --password-file, or set BACKUP_PASSWORD env var"
        exit 1
    fi

    # Check GPG is available
    if ! command -v gpg >/dev/null 2>&1; then
        echo "Error: gpg command not found. Please install GPG"
        exit 1
    fi

    ARCHIVE_FILE="arcadia_backup_${TIMESTAMP}.tar.gz.gpg"

    # Create encrypted archive
    tar -czf - -C "$(dirname "$BACKUP_DIR")" "$(basename "$BACKUP_DIR")" | \
        gpg --symmetric --cipher-algo AES256 \
            --passphrase "$BACKUP_PASSWORD" \
            --batch --yes \
            -o "$ARCHIVE_FILE"

    if [ $? -eq 0 ]; then
        ARCHIVE_SIZE=$(du -h "$ARCHIVE_FILE" | cut -f1)
        echo "Encrypted backup created: $ARCHIVE_FILE ($ARCHIVE_SIZE)"
        echo "Encryption: AES256"
    else
        echo "Error: Failed to create encrypted archive"
        exit 1
    fi
else
    ARCHIVE_FILE="arcadia_backup_${TIMESTAMP}.zip"

    if command -v zip >/dev/null 2>&1; then
        zip -rq "$ARCHIVE_FILE" "$BACKUP_DIR"
    else
        echo "Error: zip command not found. Please install zip utility"
        exit 1
    fi

    if [ $? -eq 0 ]; then
        ARCHIVE_SIZE=$(du -h "$ARCHIVE_FILE" | cut -f1)
        echo "Backup created: $ARCHIVE_FILE ($ARCHIVE_SIZE)"
    else
        echo "Error: Failed to create zip archive"
        exit 1
    fi
fi

# ============================================================================
# MOVE TO DESTINATION
# ============================================================================

if [ "$BACKUP_DESTINATION" != "." ]; then
    mkdir -p "$BACKUP_DESTINATION"
    mv "$ARCHIVE_FILE" "$BACKUP_DESTINATION/"
    ARCHIVE_FILE="$BACKUP_DESTINATION/$(basename "$ARCHIVE_FILE")"
    echo "Backup moved to: $ARCHIVE_FILE"
fi

# ============================================================================
# REMOTE UPLOAD
# ============================================================================

if [ "$REMOTE_ENABLED" = true ]; then
    echo ""
    echo "Uploading backup to remote server..."

    # Validate required parameters
    if [ -z "$REMOTE_HOST" ] || [ -z "$REMOTE_USER" ] || [ -z "$REMOTE_PATH" ]; then
        echo "Error: Remote upload requires --remote-host, --remote-user, and --remote-path"
        exit 1
    fi

    # Build SSH options
    SSH_OPTS="-o StrictHostKeyChecking=no -o ConnectTimeout=30"
    [ -n "$REMOTE_KEY" ] && SSH_OPTS="$SSH_OPTS -i $REMOTE_KEY"

    REMOTE_DEST="$REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH/"

    if [ "$REMOTE_TYPE" = "rsync" ]; then
        echo "Using rsync..."
        if [ "$REMOTE_PORT" != "22" ]; then
            rsync -avz --progress -e "ssh $SSH_OPTS -p $REMOTE_PORT" "$ARCHIVE_FILE" "$REMOTE_DEST"
        else
            rsync -avz --progress -e "ssh $SSH_OPTS" "$ARCHIVE_FILE" "$REMOTE_DEST"
        fi
        UPLOAD_STATUS=$?
    elif [ "$REMOTE_TYPE" = "scp" ]; then
        echo "Using scp..."
        if [ "$REMOTE_PORT" != "22" ]; then
            scp -P "$REMOTE_PORT" $SSH_OPTS "$ARCHIVE_FILE" "$REMOTE_DEST"
        else
            scp $SSH_OPTS "$ARCHIVE_FILE" "$REMOTE_DEST"
        fi
        UPLOAD_STATUS=$?
    else
        echo "Error: Unknown remote type '$REMOTE_TYPE'. Use 'rsync' or 'scp'"
        exit 1
    fi

    if [ $UPLOAD_STATUS -eq 0 ]; then
        echo "Remote upload successful!"
        echo "Remote location: $REMOTE_DEST$(basename "$ARCHIVE_FILE")"

        if [ "$DELETE_AFTER_UPLOAD" = true ]; then
            rm "$ARCHIVE_FILE"
            echo "Local backup deleted after successful upload"
        fi
    else
        echo "Error: Remote upload failed (exit code: $UPLOAD_STATUS)"
        exit 1
    fi
fi

# ============================================================================
# SUMMARY
# ============================================================================

echo ""
echo "============================================"
echo "Backup completed successfully!"
echo "============================================"
echo "Archive: $(basename "$ARCHIVE_FILE")"
echo "Size: $ARCHIVE_SIZE"
[ "$ENCRYPT" = true ] && echo "Encrypted: Yes (AES256)"
[ "$REMOTE_ENABLED" = true ] && echo "Uploaded to: $REMOTE_HOST:$REMOTE_PATH"
echo "Timestamp: $TIMESTAMP"
echo ""
