FROM node:22-slim AS build
WORKDIR /kiwi
RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*
ARG KIWIIRC_COMMIT=0416529fda6ddf8ab5559796ec69b49e07bd798f
RUN git init && git remote add origin https://github.com/kiwiirc/kiwiirc.git && git fetch --depth 1 origin ${KIWIIRC_COMMIT} && git checkout ${KIWIIRC_COMMIT}
RUN corepack enable && yarn install --frozen-lockfile
RUN yarn build

FROM nginx:stable-alpine-slim
COPY --from=build /kiwi/dist/ /usr/share/nginx/html/
COPY config.json /usr/share/nginx/html/static/config.json
COPY arcadia-plugin.js /usr/share/nginx/html/static/plugins/arcadia-plugin.js
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
