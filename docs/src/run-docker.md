# Docker Deployment

This guide will help you get Arcadia running quickly using Docker Compose.

## Prerequisites

- Docker and Docker Compose installed
- Git (to clone the repository)

<div class="warning">

If running `docker compose` doesn't work, you may have an older version of the docker cli installed and may need to use `docker-compose` instead.

Also don't forget to use `sudo` if you aren't in the `docker` group!
</div>

## Quick Setup

1. **Copy [configuration](configuration.md)**

    ```bash
    cp config.example.yml config.yml
    ```
    Docker Compose automatically routes inter-container traffic via environment variables, so no manual changes to `config.yml` are required to get started. You can customize site settings or pass environment variables as needed.

2. **Start services**
    ```bash
    docker compose up -d
    ```
    
    This starts the essential services:
    - PostgreSQL database (`db`) and automatic schema migrations (`init_db`)
    - Redis cache (`redis`)
    - Backend API (`backend`)
    - BitTorrent tracker (`tracker`)
    - Frontend UI and reverse proxy (`frontend`, powered by Caddy on port `5173`)

3. **Adding Test data**
    You can optionally add "fake" data (fixtures) to the database for development:
    
    ```bash
    docker compose exec -T db psql -U arcadia -d arcadia < backend/storage/migrations/fixtures/fixtures.sql
    ```
    
    Default credentials:
    - **Username**: `picolo`
    - **Password**: `test`

4. **Access the application**
    - Frontend Web UI: `http://localhost:5173`
    - Backend API: `http://localhost:5173/api/` (proxied via Caddy)

## Production Deployment

By default, Docker Compose uses development credentials (`arcadia` / `password`). For production deployments, define your secure credentials in a `.env` file at the repository root and edit the secrets:

```bash
cp example.env .env
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

### Reverse Proxy

the frontend container serves the static files of the frontend and proxies access to the other containers with [Caddy](https://caddyserver.com/).  
To make Arcadia accessible via HTTPS you will probably need a custom [`Caddyfile`](https://github.com/Arcadia-Solutions/arcadia/blob/main/frontend/Caddyfile)

In order to set this you don't need to edit the `compose.yml`. The preferred way to do those modifications is creating a [`compose.override.yml` file](compose-override.md).

## Upgrading

if you want to update your installations
In the root directory of the cloned Arcadia source:

1. Pull the latest changes:

    ```bash
    git fetch && git pull
    ```

2. Rebuild and restart the services:

    ```bash
    docker compose up -d --build
    ```

    The `init_db` container runs migrations automatically on every start, so schema changes are applied without any manual steps.

## Troubleshooting

- If services fail to start, check logs with: `docker compose logs [service-name]`
- To rebuild images: `docker compose build`
- To reset everything: `docker compose down -v && docker compose up -d`
