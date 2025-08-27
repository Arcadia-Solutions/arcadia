# Backup

This page explains how to create backups of your Arcadia installation, including the database and configuration files.

For all the possible flags and operations, check the `--help` flag:
```
./backup.sh --help
```

## Overview

The backup process includes:
- Complete database dump (schema + data)
- Environment configuration files (`.env` files)
- Backup metadata and timestamps

## Prerequisites

- `zip` utility installed on your system
- For local setup: PostgreSQL client tools (`pg_dump`)

## Quick Backup

For Docker setup:
```bash
./backup.sh --db-docker
```

For local/standard setup:
```bash
./backup.sh
```

## Backup Script Options

The backup script supports both Docker and local PostgreSQL setups with various configuration options.

### Docker Mode

Use `--db-docker` flag to backup from a containerized database:

```bash
# Default Docker backup
./backup.sh --db-docker

# Custom container name
./backup.sh --db-docker --db-container my_custom_db
```

**Docker mode options:**
- `--db-container`: Docker container name (default: `arcadia_db`)

### Local Mode

For standard installations with local PostgreSQL:

```bash
# Default local backup
./backup.sh

# Remote database
./backup.sh --db-host db.example.com --db-user myuser

# With password
./backup.sh --db-password mypassword
```

**Local mode options:**
- `--db-host`: Database host (default: `localhost`)
- `--db-port`: Database port (default: `5432`)
- `--db-name`: Database name (default: `arcadia`)
- `--db-user`: Database user (default: `arcadia`)
- `--db-password`: Database password (optional)

## Configuration Priority

The script loads configuration in this order (highest to lowest priority):

1. **Command line arguments** - Override everything
2. **Environment variables** from `.env` files:
   - Docker mode: `backend/api/.env.docker` → `backend/api/.env`
   - Local mode: `backend/api/.env`
3. **Built-in defaults**

## What Gets Backed Up

### Database
- Complete PostgreSQL dump using `pg_dump`
- Includes all schema and data
- Uses `--no-owner --no-privileges` for portability

### Configuration Files
- `backend/api/.env` → `backend.env`
- `frontend/.env` → `frontend.env`

### Metadata
- Backup timestamp
- Database information
- Setup type (Docker/Local)
- Backup configuration details

## Backup Output

The script creates:
- Temporary backup directory: `backup_YYYYMMDD_HHMMSS/`
- Final zip archive: `arcadia_backup_YYYYMMDD_HHMMSS.zip`

Example backup contents:
```
arcadia_backup_20241201_143022.zip
├── database_full.sql
├── backend.env
├── frontend.env
└── backup_info.txt
```
