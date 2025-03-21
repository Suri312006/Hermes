# Start with a Rust base image
FROM rust:1.85 AS builder

# Install the musl target
RUN rustup target add x86_64-unknown-linux-musl

# Set working directory
WORKDIR /build

# First, copy and build the agora dependency
COPY ./agora /build/agora/
COPY ./proto /build/proto/

# Then copy your application code
COPY ./sator /build/app/

# Set working directory to your app
WORKDIR /build/app

# Install protobuf compiler
RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Run the benchmarks instead of building for release
CMD ["cargo", "bench"]
