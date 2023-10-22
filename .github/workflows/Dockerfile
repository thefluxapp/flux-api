FROM rust:1 as build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim
RUN apt update
RUN apt install -y openssl
COPY --from=build /app/target/release/flux-api /app/

# TODO: Move to Alpine
