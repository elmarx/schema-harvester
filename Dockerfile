FROM rust:1.85.0 AS builder
ARG REVISION
WORKDIR /usr/src

RUN apt-get update && apt-get install -y libssl-dev libsasl2-dev

COPY Cargo.toml Cargo.lock ./
COPY cli/Cargo.toml cli/Cargo.toml
COPY core/Cargo.toml core/Cargo.toml
COPY service/Cargo.toml service/Cargo.toml

# compile all dependencies with a dummy for improved caching
RUN sed -i s/\"test-suite\",// Cargo.toml  && \
  mkdir -p cli/src core/src service/src/bin && \
  echo "fn main() { println!(\"Dummy\"); }" > service/src/bin/dummy.rs && \
  touch core/src/lib.rs && \
  cargo build --release --bin dummy && \
  rm -rf cli service

# now compile the real code
COPY cli cli
COPY core core
COPY service service

# touch to bust cargo-cache…
RUN touch core/src/lib.rs

RUN cargo install --locked --path service --root /usr/local

FROM debian:stable-slim

RUN apt-get update && apt-get install -y libssl3 libsasl2-2
ENV HARVESTER_LOGGING=json
RUN adduser --system harvester
COPY --from=builder /usr/local/bin/harvesterd /usr/local/bin

USER harvester
WORKDIR /home/harvester

CMD [ "harvesterd" ]
