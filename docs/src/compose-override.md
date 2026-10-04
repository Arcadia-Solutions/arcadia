# Customizing with Compose Override

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
