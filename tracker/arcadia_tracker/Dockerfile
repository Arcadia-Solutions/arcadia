FROM rust:1.89 AS base
RUN cargo install --locked cargo-chef

FROM base AS planner


WORKDIR /app

COPY ./backend ./backend
COPY ./shared ./shared
COPY ./tracker/arcadia_tracker ./tracker/arcadia_tracker
COPY Cargo.toml Cargo.toml

# Creates the manifest for dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
COPY --from=planner /app/backend/api/vendored /app/backend/api/vendored/

# Caches the target directory for intermediate build artifacts.
# Checks the .env.docker file to choose between dev and prod builds.
# `cargo chef cook` downloads, builds, and caches all of the dependencies for future builds.
RUN --mount=type=cache,target=target,sharing=locked if [ $(grep "^PRODUCTION_BUILD=" tracker/arcadia_tracker/.env.docker | cut -d '=' -f2) = "true" ]; then \
    cargo chef cook --release --recipe-path recipe.json; \
    else \
    cargo chef cook --recipe-path recipe.json; \
    fi

COPY . .

# The finished binary is placed in /usr/local/bin because it can't be conditionally copied from the runtime stage, so
# both prod and dev can be copied from the same place.
RUN --mount=type=cache,target=target,sharing=locked if [ $(grep "^PRODUCTION_BUILD=" tracker/arcadia_tracker/.env.docker | cut -d '=' -f2) = "true" ]; then \
    SQLX_OFFLINE=true cargo build --release && mv /app/target/release/arcadia_tracker /usr/local/bin; \
    else \
    SQLX_OFFLINE=true cargo build && mv /app/target/debug/arcadia_tracker /usr/local/bin; \
    fi

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev openssl curl pkg-config ca-certificates

COPY --from=builder /usr/local/bin/arcadia_tracker /usr/local/bin

ENTRYPOINT ["/usr/local/bin/arcadia_tracker"]
