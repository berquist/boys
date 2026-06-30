FROM rust:1.96@sha256:6df234c1eb92b0545468fab8c18fc5f9adfb994e7d4f67d81d45fe2fcabf5657 AS builder

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
