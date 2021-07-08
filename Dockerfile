FROM rust:1.51 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata openssl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

EXPOSE 3030

COPY --from=builder /app/target/release/relay-tester /app/relay-tester
RUN chmod +x /app/relay-tester

CMD ["/app/relay-tester"]