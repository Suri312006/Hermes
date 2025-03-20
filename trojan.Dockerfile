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
COPY ./trojan /build/app/
# Set working directory to your app
WORKDIR /build/app
# Install protobuf compiler
RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Build for release WITHOUT using cache mounts for target directory
# This ensures the binary is actually in the filesystem layer
RUN cargo build --target=x86_64-unknown-linux-musl --release

# Debug: confirm the binary exists
# RUN find /build/app/target -type f -executable -name "sparta" || echo "Binary not found"

# Final stage
FROM debian:bullseye-slim
# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
# Create a non-root user to run the application
# RUN groupadd -r appuser && useradd -r -g appuser appuser
# Copy the binary from builder
COPY --from=builder /build/app/target/x86_64-unknown-linux-musl/release/trojan .
# Set ownership
EXPOSE 50051
# Set the startup command
CMD ["./trojan"]
