# Image Hosting (Chevereto)

Arcadia bundles [Chevereto](https://chevereto.com/) to host user avatars, forum image attachments, and torrent media artwork (posters, covers).

The service is optional and runs in dedicated containers enabled via the `images` or `full` Docker Compose profiles.

## Setup Guide

### 1. Configure Credentials and Start Containers

In `.env`, define secure passwords for the MariaDB instance:

```bash
CHEVERETO_DB_PASS=your_secure_chevereto_password
CHEVERETO_DB_ROOT_PASS=very_secure_chevereto_root_password
```

Launch the image hosting services:

```bash
docker compose --profile images up -d
```

### 2. Run the Initial Web Installer & Generate API Key

1. Open your browser and navigate to `http://localhost:8083` (or your configured domain).
2. Complete the initial installation wizard to create your administrator account and initialize tables.
3. Once logged in as administrator, navigate to **Dashboard** &rarr; **Settings** &rarr; **API**.
4. Generate and copy the **API v1 key**.

### 3. Connect Arcadia to Chevereto

In `config.yml`, configure the `image_host:` section:

```yaml
image_host:
  chevereto_api_url: http://chevereto_php/api/1/upload
  chevereto_api_key: your_generated_chevereto_api_key
  # Automatically rehost posters and covers retrieved by scrapers (TMDB, MusicBrainz)
  rehost_external_images: true
```

*(Alternatively, inject the API key via environment variable: `ARCADIA_IMAGE_HOST__CHEVERETO_API_KEY=your_key`).*

Restart the backend container to apply the configuration:

```bash
docker compose restart backend
```

### 4. Enable Image Uploader in Staff Dashboard

By default, the frontend drag-and-drop uploader is disabled. To activate it:

1. Log in with an account having administrator permissions.
2. In the top navbar, navigate to **Staff** &rarr; **Settings**.
3. Enable **Display image upload drag and drop** and save changes.

The image upload widget will now appear when creating or editing torrents, editions, and artists. If you enforce an **Approved Image Hosts** whitelist in site settings, add your image hosting domain to the list.

## Production Reverse Proxy & HTTPS

To expose Chevereto on a dedicated subdomain in production, configure `CHEVERETO_HOSTNAME` in `compose.override.yml` and enable the Chevereto block in `Caddyfile` (both pre-configured in [`compose.override.yml.example`](https://github.com/Arcadia-Solutions/arcadia/blob/main/compose.override.yml.example) and [`Caddyfile.example`](https://github.com/Arcadia-Solutions/arcadia/blob/main/Caddyfile.example)).
