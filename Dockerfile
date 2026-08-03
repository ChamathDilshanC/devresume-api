FROM rust:1-slim AS builder


WORKDIR /app
COPY . .

RUN cargo build --release --bin api

FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api /app/devresume-api

EXPOSE 8080

CMD ["/app/devresume-api"]
