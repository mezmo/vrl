FROM rust:1.72.0 AS base

WORKDIR /opt/app

COPY Cargo.toml /opt/app/
COPY Cargo.lock /opt/app/
COPY build.rs /opt/app/
COPY data /opt/app/data
COPY benches /opt/app/benches
COPY lib /opt/app/lib
COPY src /opt/app/src
# Tests reference fixtures in this directory. Just copy everything over.
COPY tests /opt/app/tests

RUN cargo --version
RUN cargo check --locked
RUN cargo test --locked

