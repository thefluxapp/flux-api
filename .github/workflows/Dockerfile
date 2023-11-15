FROM rust:1-alpine as build

RUN apk --no-cache add musl-dev pkgconfig openssl-dev

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache upgrade
RUN apk --no-cache add gcc
COPY --from=build /app/target/release/flux-api /app/
