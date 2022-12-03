FROM rust:1.65-alpine

ENV RUSTFLAGS="-C target-feature=-crt-static"
WORKDIR /app

RUN apk add bash gcc g++ gcompat libressl-dev

# Add libtorch
ADD https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-1.13.0%2Bcpu.zip /usr/lib/libtorch.zip
# ADD https://download.pytorch.org/libtorch/nightly/cpu/libtorch-shared-with-deps-latest.zip /usr/lib/libtorch.zip
RUN unzip /usr/lib/libtorch.zip -d /usr/lib
