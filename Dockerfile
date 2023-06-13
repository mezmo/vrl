FROM rust:1.68.2 AS base

WORKDIR /opt/app

COPY Cargo.toml /opt/app/
COPY benches /opt/app/benches
COPY lib /opt/app/lib
COPY src /opt/app/src

RUN cargo check
RUN cargo test

FROM rust:1.68.2 AS wasm-base

WORKDIR /opt/app

COPY lib/lookup/Cargo.toml lib/lookup/build.rs ./
COPY lib/lookup/src ./src
COPY lib/lookup/tests ./tests
COPY lib/lookup/benches ./benches

RUN apt-get update && apt-get install nodejs --yes
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Run both rust unit tests and wasm unit tests
RUN wasm-pack test --node -- --features=path-parsing-wasm

# Then build the final artifact
RUN wasm-pack build -t nodejs --release --scope answerbook -- --features=path-parsing-wasm

FROM us.gcr.io/logdna-k8s/node-bullseye:16 AS wasm-publish
ARG GITHUB_TOKEN
ARG DRY_RUN=false

WORKDIR /opt/app

COPY --from=wasm-base /opt/app/ .
RUN touch pkg/.npmrc \
  && echo "@answerbook:registry=https://npm.pkg.github.com/" >> pkg/.npmrc \
  && echo '//npm.pkg.github.com/:_authToken=${GITHUB_TOKEN}' >> pkg/.npmrc

RUN if [ ${DRY_RUN} = true ]; then \
        echo 'Dry running publish...' && \
        cd pkg && npm publish --dry-run; \
    else \
        echo 'Publishing...' && \
        cd pkg && npm publish; \
    fi
RUN rm pkg/.npmrc
