FROM rust:1.71-alpine

# ENV RUSTFLAGS="-C target-feature=-crt-static"
WORKDIR /app

RUN apk add zsh gcc g++ libressl-dev
