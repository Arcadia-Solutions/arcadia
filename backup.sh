#!/bin/bash

# Arcadia Backup Script
# This script backs up the database data and the configuration file
# Supports both Docker and non-Docker setups
#
# Usage:
#   ./backup.sh [--db-docker] [--db-name NAME] [--db-user USER] [--db-container CONTAINER]
#   --db-docker: Use Docker setup (default: local PostgreSQL)
#   --db-name: Database name (default: arcadia)
#   --db-user: Database user (default: arcadia)
#   --db-container: Docker container name (default: arcadia_db)

set -e  # Exit on any error

# Default to local setup
USE_DOCKER=false

# Parse command line arguments first (to handle --help before loading config)
while [[ $# -gt 0 ]]; do
    case $1 in
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
        -h|--help)
            echo "Arcadia Backup Script - Creates a complete backup of the application"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "BACKUP MODES:"
            echo "  --db-docker      Use Docker setup (connects to database container)"
            echo "  (default)        Use local setup (connects to local PostgreSQL)"
            echo ""
            echo "DOCKER MODE OPTIONS:"
            echo "  --db-container   Docker container name (default: arcadia_db)"
            echo "                   Override if your container has a different name"
            echo ""
            echo "LOCAL MODE OPTIONS:"
            echo "  --db-host        Database host (default: localhost)"
            echo "  --db-port        Database port (default: 5432)"
            echo "  --db-name        Database name (default: arcadia)"
            echo "  --db-user        Database user (default: arcadia)"
            echo "  --db-password    Database password (optional)"
            echo "                   If not provided, uses .pgpass, environment vars, or peer auth"
            echo ""
            echo "COMMON OPTIONS:"
            echo "  -h, --help       Show this help message"
            echo ""
            echo "EXAMPLES:"
            echo "  $0                           # Local backup with default settings"
            echo "  $0 --db-docker               # Docker backup with default container"
            echo "  $0 --db-host db.example.com  # Local backup to remote database"
            echo "  $0 --db-docker --db-container my_db  # Docker backup with custom container"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Load configuration after argument parsing
# Default configuration
# Default values will be set later after processing command line arguments and environment variables
BACKUP_DIR="backup_$(date +%Y%m%d_%H%M%S)"
ZIP_FILE="arcadia_backup_$(date +%Y%m%d_%H%M%S).zip"

# The whole project is configured by a single config.yml at the root of the repository.
ARCADIA_CONFIG="${ARCADIA_CONFIG:-config.yml}"
CONFIG_FILE="$ARCADIA_CONFIG"

. "$(dirname "$0")/scripts/config_value.sh"

if [ -f "$CONFIG_FILE" ]; then
    echo "Loading configuration from $CONFIG_FILE..."
    POSTGRES_DATABASE=$(config_value database name)
    POSTGRES_USER=$(config_value database user)
    POSTGRES_PASSWORD=$(config_value database password)
    POSTGRES_HOST=$(config_value database host)
    POSTGRES_PORT=$(config_value database port)
fi

# Function to strip carriage returns from variables
strip_cr() {
    echo "$1" | tr -d '\r'
}

# Map the values read from the configuration file to DB_ variables (only if not set by command line)
# Command line arguments take highest priority
if [ -z "$DB_NAME" ]; then
    DB_NAME=$(strip_cr "${POSTGRES_DATABASE:-"arcadia"}")
fi
if [ -z "$DB_USER" ]; then
    DB_USER=$(strip_cr "${POSTGRES_USER:-"arcadia"}")
fi
if [ -z "$DB_PASSWORD" ]; then
    DB_PASSWORD=$(strip_cr "${POSTGRES_PASSWORD}")
fi
if [ -z "$DB_HOST" ]; then
    DB_HOST=$(strip_cr "${POSTGRES_HOST:-"localhost"}")
fi
if [ -z "$DB_PORT" ]; then
    DB_PORT=$(strip_cr "${POSTGRES_PORT:-"5432"}")
fi
if [ -z "$DB_CONTAINER" ]; then
    DB_CONTAINER=$(strip_cr "${DB_CONTAINER:-"arcadia_db"}")
fi
BACKUP_DIR=${BACKUP_DIR:-"backup_$(date +%Y%m%d_%H%M%S)"}
ZIP_FILE=${ZIP_FILE:-"arcadia_backup_$(date +%Y%m%d_%H%M%S).zip"}

echo "Starting Arcadia backup..."

# Create temporary backup directory
mkdir -p "$BACKUP_DIR"

# Function to cleanup on exit
cleanup() {
    echo "Cleaning up temporary files..."
    rm -rf "$BACKUP_DIR"
}
trap cleanup EXIT

# Use Docker or local setup based on USE_DOCKER flag
if [ "$USE_DOCKER" = true ]; then
    echo "Using Docker setup - containerized database"
    # Check if Docker is running
    if ! docker info >/dev/null 2>&1; then
        echo "Error: Docker is not running or not accessible"
        exit 1
    fi
    # Check if database container is running
    if ! docker ps --format "table {{.Names}}" | grep -q "^$DB_CONTAINER$"; then
        echo "Error: Database container '$DB_CONTAINER' is not running"
        echo "Please start the database with: docker compose up db -d"
        exit 1
    fi
else
    echo "Using local setup - local PostgreSQL installation"
    # Check if the configuration file exists
    if [ ! -f "$CONFIG_FILE" ]; then
        echo "Error: $CONFIG_FILE not found"
        echo "Please copy config.example.yml to config.yml and fill in the database section"
        exit 1
    fi
fi

echo "Backing up database..."

if [ "$USE_DOCKER" = true ]; then
    # Docker setup - use docker exec
    echo "Using Docker container for database backup..."
    docker exec "$DB_CONTAINER" pg_dump -U "$DB_USER" -d "$DB_NAME" --no-owner --no-privileges > "$BACKUP_DIR/database_full.sql"
    BACKUP_EXIT_CODE=$?
else
    # Local setup - use local pg_dump with DATABASE_URL
    echo "Using local PostgreSQL installation for database backup..."
    
    # Check if pg_dump is available
    if ! command -v pg_dump >/dev/null 2>&1; then
        echo "Error: pg_dump command not found. Please install PostgreSQL client tools"
        exit 1
    fi
    
    # Use pg_dump with individual connection parameters
    echo "Connecting to database: $DB_HOST:$DB_PORT/$DB_NAME as $DB_USER"
    if [ -n "$DB_PASSWORD" ]; then
        PGPASSWORD="$DB_PASSWORD" pg_dump \
            -h "$DB_HOST" \
            -p "$DB_PORT" \
            -U "$DB_USER" \
            -d "$DB_NAME" \
            --no-owner --no-privileges > "$BACKUP_DIR/database_full.sql"
    else
        # No password provided, rely on .pgpass, environment, or peer authentication
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
    echo "Database backup completed successfully"
else
    echo "Error: Database backup failed"
    exit 1
fi

echo "Copying the configuration file..."
if [ -f "$CONFIG_FILE" ]; then
    cp "$CONFIG_FILE" "$BACKUP_DIR/config.yml"
    echo "Configuration file copied"
else
    echo "Warning: $CONFIG_FILE not found, skipping..."
fi

# Create backup info file
echo "Backup created on: $(date)" > "$BACKUP_DIR/backup_info.txt"
echo "Database: $DB_NAME" >> "$BACKUP_DIR/backup_info.txt"
if [ "$USE_DOCKER" = true ]; then
    echo "Setup type: Docker" >> "$BACKUP_DIR/backup_info.txt"
    echo "Container: $DB_CONTAINER" >> "$BACKUP_DIR/backup_info.txt"
else
    echo "Setup type: Local PostgreSQL" >> "$BACKUP_DIR/backup_info.txt"
    echo "Database URL: $(echo $DATABASE_URL | sed 's/:.*@/:***@/')" >> "$BACKUP_DIR/backup_info.txt"
fi
echo "Backup type: Full dump (schema + data)" >> "$BACKUP_DIR/backup_info.txt"

echo "Creating zip archive..."
# Create zip file
if command -v zip >/dev/null 2>&1; then
    zip -r "$ZIP_FILE" "$BACKUP_DIR"
else
    echo "Error: zip command not found. Please install zip utility"
    exit 1
fi

if [ $? -eq 0 ]; then
    echo "Backup completed successfully!"
    echo "Backup file: $ZIP_FILE"
    echo "Backup size: $(du -h "$ZIP_FILE" | cut -f1)"
else
    echo "Error: Failed to create zip archive"
    exit 1
fi

echo "Backup process finished."