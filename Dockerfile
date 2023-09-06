FROM rust:1.72.0 AS base

WORKDIR /opt/app

COPY Cargo.toml /opt/app/
COPY benches /opt/app/benches
COPY lib /opt/app/lib
COPY src /opt/app/src

RUN cargo --version
RUN cargo check
RUN cargo test

