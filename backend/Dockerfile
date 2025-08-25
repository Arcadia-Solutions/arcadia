FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
COPY api/vendored vendored

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 

COPY --from=planner /app/recipe.json recipe.json
COPY --from=planner /app/vendored /app/api/vendored/

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN SQLX_OFFLINE=true cargo build --release --bin arcadia-api

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl-dev openssl curl pkg-config ca-certificates
WORKDIR /app
COPY --from=builder /app/target/release/arcadia-api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/arcadia-api"]
