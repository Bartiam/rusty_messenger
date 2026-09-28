FROM rust:1.98.0 AS builder
WORKDIR /app

COPY . .
COPY .sqlx .sqlx
ENV SQLX_OFFLINE=true

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM debian:trixie-slim
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rusty_messenger /app/rusty_messenger

EXPOSE 8080

CMD ["./rusty_messenger"]
