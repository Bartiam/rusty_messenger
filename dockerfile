FROM rust:1.97.1 AS builder
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/rusty_messenger /app/rusty_messenger

EXPOSE 3000

CMD ["./rusty_messenger"]