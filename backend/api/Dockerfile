FROM debian:bookworm-slim AS debian-base

RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

FROM alpine:3 AS alpine-base

FROM rust:1.89-slim-bookworm AS prebuild
# Don't delete any downloaded packages, we'd be using BuildKit cache mounts later!
RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

WORKDIR /app
# COPY . .
COPY .sqlx/ ./sqlx/
COPY vendored vendored

COPY Cargo.toml Cargo.lock ./

# This doesn't mount anything from your system, it's all in the build cache
# Helps speed up repetitive docker builds by A LOT
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    apt-get update && apt-get install -y libssl-dev openssl curl pkg-config ca-certificates

COPY migrations/ migrations/

RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

# The dummy file is our only way to build only the dependencies before the app itself.
RUN mkdir ./src && \
    echo "fn main() {}" > ./src/dummy.rs

FROM prebuild AS build-prod

RUN mv ./src/dummy.rs ./src/main.rs

RUN SQLX_OFFLINE=true cargo build --release

RUN rm ./src/main.rs
COPY . .

RUN SQLX_OFFLINE=true cargo build --release

FROM prebuild AS build-debug

RUN mv ./src/dummy.rs ./src/main.rs

RUN SQLX_OFFLINE=true cargo build

RUN rm ./src/main.rs
COPY . .

RUN SQLX_OFFLINE=true cargo build

FROM alpine-base AS debug-alpine

RUN apk add --no-cache openssl

WORKDIR /app

COPY --chmod=777 --from=build-debug /app/target/debug/arcadia-backend .
CMD ["./arcadia-backend"]

FROM alpine-base AS alpine

RUN apk add --no-cache openssl

WORKDIR /app

COPY --chmod=777 --from=build-prod /app/target/release/arcadia-backend .
CMD ["./arcadia-backend"]

# use --target debug when you want to build the debug variant
FROM debian-base AS debug
WORKDIR /app

COPY --chmod=755 --from=build-debug /app/target/debug/arcadia-backend .
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && apt-get install -y openssl ca-certificates

CMD ["./arcadia-backend"]

FROM debian-base
WORKDIR /app

COPY --chmod=755 --from=build-prod /app/target/release/arcadia-backend .
# COPY --from=prebuild /usr/local/cargo/bin/sqlx .
# COPY  ./migrations ./migrations

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && apt-get install -y openssl ca-certificates

CMD ["./arcadia-backend"]
