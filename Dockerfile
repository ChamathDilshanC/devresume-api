FROM rust:1-slim AS builder

WORKDIR /app

# Install system build dependencies required by OpenSSL & Cargo dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release --bin api

FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api /app/devresume-api

# Azure Container Apps (and most PaaS hosts) route to a fixed target port.
# The app reads PORT from the environment (see crates/common/src/config.rs),
# so this can be overridden at deploy time with `--env-vars PORT=<port>` if needed.
ENV PORT=8080

EXPOSE 8080

CMD ["/app/devresume-api"]
