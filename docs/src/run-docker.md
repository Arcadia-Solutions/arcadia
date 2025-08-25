# Docker Setup

This guide will help you get Arcadia running quickly using Docker Compose.

## Prerequisites

- Docker and Docker Compose installed
- Git (to clone the repository)

<div class="warning">

If running `docker compose` doesn't work, you may have an older version of the docker cli installed and may need to use `docker-compose` instead.

Also don't forget to use `sudo` if you aren't in the `docker` group!
</div>

## Quick Setup

1. **Set up environment files**:
   ```bash
   # Copy backend api environment file
   cp backend/api/.env.docker backend/api/.env

   # Copy frontend environment file
   cp frontend/.env.docker frontend/.env
   ```

2. **Start all services**:
   ```bash
   docker compose up -d
   ```

   This command will:
   - Build the backend  and frontend images
   - Start PostgreSQL database
   - Run database migrations automatically
   - Start the backend API server
   - Start the frontend development server

3. **Access the application**:
   - Frontend: `http://localhost:5137`
   - Backend API: `http://localhost:8080`

## Individual Service Management

If you prefer to start services individually:

### Database Only
   ```bash
   docker compose up db -d
   ```

### Backend Api Only
   ```bash
   docker compose up backend -d
   ```

### Frontend Only
   ```bash
   docker compose up frontend -d
   ```

## Development Features

### Auto-rebuild with Compose Watch

For development, you can use [Compose Watch](https://docs.docker.com/compose/how-tos/file-watch/) to automatically rebuild when source code changes:

```bash
docker compose up --watch
```

Or when running attached (without `-d`), press <kbd>W</kbd> to enable watch mode.

### Adding Test Data

You can optionally add "fake" data (fixtures) to the database for development:

```bash
docker exec -i arcadia_db psql -U arcadia -d arcadia < backend/storage/migrations/fixtures/fixtures.sql
```

The default test user is `picolo` with password `test`.

### Exporting Test Data

If you added some new test data and wish to include it in your commit, you can export it like so:

```bash
docker exec -i arcadia_db pg_dump -U arcadia -d arcadia --data-only --inserts > backend/storage/migrations/fixtures/fixtures.sql
```

## Manual Database Setup (if needed)

Arcadia automatically runs migrations on launch, but if you need to manually set up the database:

```bash
cargo install sqlx-cli
cargo sqlx database setup
```

## Troubleshooting

- If services fail to start, check logs with: `docker compose logs [service-name]`
- To rebuild images: `docker compose build`
- To reset everything: `docker compose down -v && docker compose up -d`
