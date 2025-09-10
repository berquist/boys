FROM rust:1.89@sha256:85456cda22d5d265c209d6e8d053e9b23de40424f518620f9e75f95e3484f609 AS builder

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
