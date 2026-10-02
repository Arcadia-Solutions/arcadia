# Running Arcadia

There are two main ways to run Arcadia:

## Configuration

The whole project is configured by a **single `config.yml` file at the root of the repository**:
backend, tracker, periodic tasks and frontend all read it. It is git ignored.

   ```bash
   cp config.example.yml config.yml
   ```

Then edit `config.yml` with the values you want. `config.example.yml` is the reference: it
documents every key, and the ones that differ under Docker carry a `docker:` note giving the
value to use.

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

### additional config files

Some of the services used with Arcadia need their own config files.
kiwiirc and ergo are not required to run the rest of Arcadia

   ```bash
   cp kiwiirc/config.json.example kiwiirc/config.json # optional (uses example as fallback automatically)
   cp ergo/ergo.motd.example ergo/ergo.motd
   cp ergo/ergo-conf.yaml.example ergo/ergo-conf.yaml
   ```

The API tokens declared in `ergo/ergo-conf.yaml` must match the ones of the `ergo` section of
`config.yml`.

## Setup Methods

### Standard Setup
Install dependencies directly on your system. See [Standard Setup](run-standard.md) for detailed instructions.

### Docker Setup
Use containerized deployment with Docker Compose. See [Docker Setup](run-docker.md) for detailed instructions.
