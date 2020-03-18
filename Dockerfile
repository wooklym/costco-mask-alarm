FROM rust:1.41 as build

RUN USER=root cargo new --bin app
WORKDIR /app

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo build --release

FROM debian:buster-slim

# install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates

COPY --from=build /app/target/release/mask .
COPY .env .env

CMD ./mask
