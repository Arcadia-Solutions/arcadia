# Getting Started

Arcadia can be deployed using Docker Compose (recommended) or directly on bare metal.

## 1. Clone Repository

```bash
git clone https://github.com/Arcadia-Solutions/arcadia.git
cd arcadia
```

## 2. Choose Deployment Method

Review the [Configuration Reference](configuration.md) for secret and environment variable details, then choose your deployment route:

- **[Docker Deployment](run-docker.md)** (Recommended): Containerized setup with bundled PostgreSQL, Redis, Caddy reverse proxy, and automatic migrations.
- **[Bare-Metal Installation](run-bare-metal.md)**: Manual setup running directly on the host system.
