FROM rust:1.64-alpine

ENV RUSTFLAGS="-C target-feature=-crt-static"
WORKDIR /app

RUN apk add bash gcc g++ libressl-dev
