# Configuration

The whole project is configured by a **single `config.yml` file at the root of the repository**:
backend, tracker, periodic tasks and frontend all read it. It is git ignored.

   ```bash
   cp config.example.yml config.yml
   cp example.env .env
   ```

Then edit `config.yml` with the values you want. [`config.example.yml`](https://github.com/Arcadia-Solutions/arcadia/blob/main/config.example.yml) is the reference you can always compare your configuration against.  
[`.env`](https://github.com/Arcadia-Solutions/arcadia/blob/main/example.env) is used to override the default credentials used by multiple services not configured by `config.yml` (e.g. Databases), so there's one place to set the correct values.

For a quick local setup, you already have a working configuration. But in a [production environment](run-docker#production) (other people access it) you will need to change all of the secrets.

## Environment Variables

Any setting in `config.yml` can be overridden via environment variables using:
`ARCADIA_<SECTION>__<KEY>` (single underscore after `ARCADIA_`, double underscore `__` between sections and keys).

Environment variables always take precedence over values in `config.yml`. If `config.yml` is absent, services can run completely fileless using environment variables and default values alone.

Common examples:

| Setting | YAML Key | Environment Variable |
| :--- | :--- | :--- |
| Database password | `database.password` | `ARCADIA_DATABASE__PASSWORD=secret` |
| Database host | `database.host` | `ARCADIA_DATABASE__HOST=db` |
| Redis password | `redis.password` | `ARCADIA_REDIS__PASSWORD=secret` |
| Redis host | `redis.host` | `ARCADIA_REDIS__HOST=redis` |
| API host & port | `api.host`, `api.port` | `ARCADIA_API__HOST=0.0.0.0`, `ARCADIA_API__PORT=8080` |
| Tracker host & port | `tracker.host`, `tracker.port` | `ARCADIA_TRACKER__HOST=0.0.0.0`, `ARCADIA_TRACKER__PORT=8081` |
| JWT Secret | `api.jwt_secret` | `ARCADIA_API__JWT_SECRET=supersecret` |

## Other Customization

A few things need to be setup outside of `config.yml`.

### Landing page

Arcadia allows you to display a custom landing page for not logged in users.
If `frontend.enable_custom_front_page` is set to `true` in `config.yml`, the file `public/home/index.html` will be served when visiting root url.

### Unauthenticated pages

The pages reachable without being signed in (`/login`, `/register`, `/apply` and `/reset-password`) can't
use the css sheets nor the custom js of the public arcadia settings, since both are tied to a signed in
user. Two optional files, git ignored, are loaded instead:

- `frontend/public/custom_unauth.css`: css applied to those pages
- `frontend/public/custom_unauth.js`: js executed on those pages

Simply leaving them out is a valid setup. Creating or editing them requires rebuilding the frontend.

### Assets

A few assets need to be setup.

- `frontend/src/assets/logo.svg`: The logo of the site (optional, defaults to `logo.example.svg`)
- `frontend/public/favicon.ico`: The favicon for the website
- `frontend/public/default_user_avatar.png`: The default avatar for users who didn't set one
- `frontend/public/bonus_points_icon.png`: The icon for bonus points

## additional config files

Some of the services used with Arcadia need their own config files.

if you don't want to run an integrated [IRC Server](irc-server.md) you can leave them as is
