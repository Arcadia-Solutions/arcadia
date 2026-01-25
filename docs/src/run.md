# Running Arcadia

There are two main ways to run Arcadia:

## Environment Configuration

   ```bash
   # Copy example environment files and edit them with the values you want
   cp backend/api/.env.example backend/api/.env
   cp backend/storage/.env.example backend/storage/.env
   cp backend/periodic-tasks/.env.example backend/periodic-tasks/.env
   cp frontend/.env.example frontend/.env
   cp shared/.env.example shared/.env
   cp tracker/.env.example tracker/.env
   ```

## Other Customization

A few things need to be setup outside of the env variables.

### Landing page

Arcadia allows you to display a custom landing page for not logged in users.
If `VITE_ENABLE_CUSTOM_FRONT_PAGE` is set to `true` in the frontend `.env` file, the file `public/home/index.html` will be served when visiting root url.

### Assets

A few assets need to be setup.

- `frontend/public/favicon.ico`: The favicon for the website
- `frontend/public/default_user_avatar.png`: The default avatar for users who didn't set one
- `frontend/public/bonus_points_icon.png`: The icon for bonus points

## Setup Methods

### Standard Setup
Install dependencies directly on your system. See [Standard Setup](run-standard.md) for detailed instructions.

### Docker Setup
Use containerized deployment with Docker Compose. See [Docker Setup](run-docker.md) for detailed instructions.
