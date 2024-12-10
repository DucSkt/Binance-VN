FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN SQLX_OFFLINE=true cargo build --release --bin binance

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt install -y openssl ca-certificates
WORKDIR /app
COPY .env /app/.env
#COPY --from=builder /app/config/default.toml config/default.toml
COPY --from=builder /app/target/release/binance /usr/local/bin
ENTRYPOINT ["/usr/local/bin/binance"]
