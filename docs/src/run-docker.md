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

### 1. Copy configuration

   ```bash
   cp config.example.yml config.yml
   ```

Docker Compose automatically routes inter-container traffic via environment variables, so no manual changes to `config.yml` are required to get started. You can customize site settings or pass environment variables as needed.

### 2. Start services
```bash
docker compose up -d
```

This starts the essential services:
- PostgreSQL database (`db`) and automatic schema migrations (`init_db`)
- Redis cache (`redis`)
- Backend API (`backend`)
- BitTorrent tracker (`tracker`)
- Frontend UI and reverse proxy (`frontend`, powered by Caddy on port `5173`)

### 3. Adding Test data

You can optionally add "fake" data (fixtures) to the database for development:

```bash
docker compose exec -T db psql -U arcadia -d arcadia < backend/storage/migrations/fixtures/fixtures.sql
```

Default credentials:
- **Username**: `picolo`
- **Password**: `test`

### 4. Access the application
- **Frontend Web UI**: `http://localhost:5173`
- **Backend API**: `http://localhost:5173/api/` (proxied via Caddy)

---

## Production Database & Service Credentials (`.env`)

By default, Docker Compose uses development credentials (`arcadia` / `password`). For production deployments, define your secure credentials in a `.env` file at the repository root:

```bash
cp .env.example .env
```

Edit `.env`:
```ini
# Database credentials
DB_USER=arcadia
DB_PASSWORD=your_secure_db_password
DB_NAME=arcadia

# Redis cache password
REDIS_PASSWORD=your_secure_redis_password
```

Docker Compose automatically loads this file and propagates the variables across the stack:
- **`db`**: Configures PostgreSQL user, password, database, and healthcheck.
- **`init_db`**: Injects `DATABASE_URL` for running schema migrations.
- **`redis`**: Configures Redis server password authentication (`--requirepass`).
- **`backend` & `tracker`**: Automatically injects database and Redis credentials via environment variables, overriding default values from `config.yml`.

> [!NOTE]
> PostgreSQL only uses `POSTGRES_PASSWORD` when initializing a new database cluster. If you change `DB_PASSWORD` on an existing installation after the `db_data` volume has already been initialized, you must also update the password inside PostgreSQL:
> ```bash
> docker exec -it arcadia_db psql -U arcadia -d arcadia -c "ALTER USER arcadia WITH PASSWORD 'new_password';"
> ```

---

## Full Stack (Optional Services)

By default, only the core services are started. To run all optional services (OpenTelemetry, Grafana dashboards, Ergo IRC server, KiwiIRC webchat, Chevereto image host):

```bash
docker compose --profile full up -d
```

You can also enable individual components by profile:
- IRC only: `docker compose --profile irc up -d`
- Telemetry & Grafana only: `docker compose --profile telemetry up -d`
- Image hosting only: `docker compose --profile images up -d`


---

## Customizing with Compose Override

Docker Compose automatically detects and merges `compose.override.yml` with `compose.yml`. Use this to adjust ports, volumes, or environment variables without modifying the version-controlled `compose.yml`.

Create a `compose.override.yml` at the repository root:

### Common Override Scenarios:

#### 1. Expose standard HTTP/HTTPS ports (Production)
```yaml
services:
  frontend:
    ports:
      - "80:80"
      - "443:443"
      - "443:443/udp"
    volumes:
      - ./Caddyfile:/etc/caddy/Caddyfile:ro # custom Caddyfile with your domain setup
      - caddy_data:/data # needed for saving certificate data
volumes:
  caddy_data:
```



#### 2. Expose internal database/redis ports for host debugging (Development)
```yaml
services:
  db:
    ports:
      - "5432:5432"
  redis:
    ports:
      - "6379:6379"
  backend:
    ports:
      - "8080:8080"
```

#### 3. Override configuration with environment variables
You can override any setting from `config.yml` using `environment:` blocks:
```yaml
services:
  backend:
    environment:
      ARCADIA_API__LOG_LEVEL: "debug,sqlx=debug"
      ARCADIA_API__JWT_SECRET: "my-secure-production-secret"
  tracker:
    environment:
      ARCADIA_TRACKER__NUMWANT: "30"
```

---

## Development Features

### Auto-rebuild with Compose Watch

For live development, Compose Watch automatically rebuilds images or syncs frontend files on source changes:

```bash
docker compose up --watch
```

Or when running attached without `-d`, press <kbd>W</kbd> to enable watch mode.

### Exporting Test Data

If you added new test data in your local container and wish to update the repository fixtures:

```bash
docker compose exec -T db pg_dump -U arcadia -d arcadia --data-only --inserts --column-inserts > backend/storage/migrations/fixtures/fixtures.sql && sed -i '/SELECT pg_catalog.set_config(\x27search_path\x27, \x27\x27, false);/d' backend/storage/migrations/fixtures/fixtures.sql
```

1 line generated by `pgdump` must be removed as it prevents the `collage_entry` fixtures from being inserted (the trigger somehow can't be interprted). If someone has an explanation, please let us know/open a PR!

## Manual Database Setup (if needed)

Arcadia automatically runs migrations on launch (`init_db` container), but if you need to manually run migrations against a running database:

```bash
cargo install sqlx-cli
DATABASE_URL=postgresql://arcadia:password@localhost:5432/arcadia cargo sqlx database setup
```

`sqlx-cli` only reads `DATABASE_URL`, it does not know about `config.yml`. Use the credentials of
the `database` section.
Make sure the database port is exposed.

## Troubleshooting

- If services fail to start, check logs with: `docker compose logs [service-name]`
- To rebuild images: `docker compose build`
- To reset everything: `docker compose down -v && docker compose up -d`
