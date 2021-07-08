FROM rust:1.51 as builder

WORKDIR /app
COPY Cargo.* .
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build
COPY src .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app

EXPOSE 3030

COPY --from=builder /app/target/release/relay-tester /app/relay-tester

CMD ["/app/relay-tester"]