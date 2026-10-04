# Integrations

## Available Services

- **IRC Server**  
    Arcadia has tight integration with [ergo](https://ergo.chat/about/) (server) and [kiwi](https://kiwiirc.com/) (web-UI)
    See the Instructions for [configuring them for your Setup](irc-server.md)

- **Image Host**  
    [Chevereto](https://chevereto.com/) is bundled to be used as the image host for your Site

- **OpenTelemetry**  
    You can use the [OpenTelemetry Stack](https://opentelemetry.io/) to collect various metrics (Performance, Error sources, etc.) and create Dashboards monitoring your Installation.

## Run Optional Services

By default, only the core services are started with `docker compose up -d`.
To run all optional services (OpenTelemetry, Grafana dashboards, Ergo IRC server, KiwiIRC webchat, Chevereto image host), you can enable the full profile:

```bash
docker compose --profile full up -d
```

You can also enable individual components by profile:
- IRC only: `docker compose --profile irc up -d`
- Telemetry & Grafana only: `docker compose --profile telemetry up -d`
- Image hosting only: `docker compose --profile images up -d`
