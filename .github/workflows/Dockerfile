FROM rust:1-alpine as build

RUN apk update
RUN apk upgrade
RUN apk add musl-dev pkgconfig openssl-dev
ENV OPENSSL_DIR=/usr

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
RUN apk update
RUN apk upgrade
COPY --from=build /app/target/release/flux-api /app/
