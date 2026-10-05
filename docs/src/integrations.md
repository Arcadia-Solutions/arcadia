# Integrations Overview

Arcadia provides optional bundled services managed through Docker Compose profiles. By default, running `docker compose up -d` starts only the core services (`db`, `init_db`, `redis`, `backend`, `tracker`, `frontend`).

To activate optional integrations, specify their profile:

| Profile | Command | Services Started | Description |
| :--- | :--- | :--- | :--- |
| **All Services** | `docker compose --profile full up -d` | All containers | Starts the complete stack including IRC, Chevereto, and Grafana. |
| **[IRC Chat](irc-server.md)** | `docker compose --profile irc up -d` | `ergo`, `ergo_database` | Ergo IRC daemon with KiwiIRC web client and MariaDB history. |
| **[Image Hosting](image-host.md)** | `docker compose --profile images up -d` | `chevereto_php`, `chevereto_database` | Chevereto image hosting platform for avatars and torrent media. |
| **[Telemetry](telemetry.md)** | `docker compose --profile telemetry up -d` | `otel-lgtm`, `hostmetrics` | OpenTelemetry collector and pre-built Grafana dashboards. |

Profiles can be combined:
```bash
docker compose --profile irc --profile images up -d
```
