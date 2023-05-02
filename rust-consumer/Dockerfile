FROM rust:1.58 AS builder

WORKDIR /usr/src

COPY . rust-consumer/
WORKDIR /usr/src/rust-consumer

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/rust-consumer/target/release/rust-consumer /usr/local/bin/

CMD ["rust-consumer"]

