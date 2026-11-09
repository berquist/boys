FROM rust:1.99@sha256:8a4e90c7946b74626488b1446f712c0fe0ed1a596cbd3d871684bc9c5433ae07 AS builder

LABEL org.opencontainers.image.source=https://github.com/berquist/boys
LABEL org.opencontainers.image.description="build image for boys Rust crate"
LABEL org.opencontainers.image.licenses=GPL-3.0-only

WORKDIR /code/boys
COPY . .
RUN \
    --mount=type=cache,target=/code/boys/target,sharing=locked \
    cargo test \
    && cargo build --release
