# Use the official Rust image as the base image
FROM rust:1.83.0-alpine3.21

# Install required build dependencies
RUN apk add --no-cache \
    musl-dev \
    gcc \
    make \
    libc-dev

# Create a new directory for the application
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Set the startup command to run the binary
ENTRYPOINT [ "./target/release/token_metadata" ]