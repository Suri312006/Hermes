# Start with a Rust base image
FROM rust:1.85 AS builder

# Set working directory
WORKDIR /build

# First, copy and build the agora dependency
COPY ./agora /build/agora/

# Then copy your application code
COPY ./sparta /build/app/

# Set working directory to your app
WORKDIR /build/app



# Install protobuf compiler
RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Build for release with dependency caching
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/app/target \
    cargo build --target=x86_64-unknown-linux-musl --release

# Final stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user to run the application
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Copy the binary from builder
# Adjust the binary name to match your application's name in Cargo.toml

COPY --from=builder /build/app/target/x86_64-unknown-linux-musl/release/sparta /usr/local/bin/sparta
# Set ownership
RUN chown appuser:appuser /usr/local/bin/sparta

# Switch to non-root user
USER appuser

# Expose the gRPC port
EXPOSE 50051

# Set the startup command
CMD ["sparta"]
