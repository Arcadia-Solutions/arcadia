ARG NODE_VERSION=lts

FROM node:${NODE_VERSION}-slim AS build

WORKDIR /home/node/app

# The build context is the repository root.
COPY frontend/ ./frontend/

WORKDIR /home/node/app/frontend

RUN --mount=type=cache,target=/root/.npm \
    npm ci --verbose --no-audit
# The frontend is configured by the `frontend` section of the root configuration file, which vite
# inlines in the bundle at build time. It is mounted as a secret rather than copied, so that the
# rest of it (jwt secret, database password, api keys) does not end up in an image layer.
# This should be npm run build
RUN --mount=type=secret,id=arcadia_config,target=/home/node/app/config.yml \
    npx vite build

FROM nginx:stable-alpine-slim

COPY --from=build /home/node/app/frontend/dist/ /usr/share/nginx/html
COPY frontend/docker/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
