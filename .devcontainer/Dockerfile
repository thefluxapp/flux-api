FROM rust:1-slim

RUN apt -y update
RUN apt -y install git
RUN rustup component add rustfmt rust-analyzer
RUN cargo install cargo-watch
