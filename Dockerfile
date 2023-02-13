FROM rust:1.67-alpine

ENV RUSTFLAGS="-C target-feature=-crt-static"
WORKDIR /app

RUN apk add bash gcc g++ libressl-dev
