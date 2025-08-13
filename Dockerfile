FROM rust:1.89@sha256:5fa1490d5cd16725196511190baad604ddebedcd6e52f1036de46a1a75c85bce AS builder

LABEL org.opencontainers.image.source=https://github.com/berquist/boys
LABEL org.opencontainers.image.description="build image for boys Rust crate"
LABEL org.opencontainers.image.licenses=GPL-3.0-only

ENV DEBIAN_FRONTEND=noninteractive

# hadolint ignore=DL3008
RUN \
    --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update \
    && apt-get install -y --no-install-recommends \
      libgsl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /code/boys
COPY . .
RUN \
    --mount=type=cache,target=/code/boys/target,sharing=locked \
    cargo test \
    && cargo build --release
