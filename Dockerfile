# BUILD CONTAINER
FROM rust:slim AS builder
WORKDIR /app

# Install build dependencies
RUN apt update && apt install -y pkg-config libssl-dev

# Build rust dependencies
RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source code into the container
COPY src ./src
RUN touch -a -m ./src/main.rs

# Build the source code
RUN cargo build --release


# RUNTIME CONTAINER
FROM gcr.io/distroless/cc AS runtime
WORKDIR /app

# Copy the static files
COPY static ./static

# Copy the built binary from the builder container
COPY --from=builder /app/target/release/status-page ./

# Run the binary
CMD ["/app/status-page"]