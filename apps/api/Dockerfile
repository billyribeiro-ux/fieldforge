# ── Build Stage ──
FROM rust:1.75-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
ENV SQLX_OFFLINE=true
RUN cargo build --release && rm -rf src target/release/deps/fieldforge*

# Build actual binary
COPY . .
RUN cargo build --release

# ── Runtime Stage ──
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/fieldforge-api /app/fieldforge-api

EXPOSE 8080

ENV API_HOST=0.0.0.0
ENV API_PORT=8080
ENV RUST_LOG=fieldforge_api=info,tower_http=info

CMD ["/app/fieldforge-api"]
