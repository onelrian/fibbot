# Use the official Rust image
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy source files
COPY . .

# Build the Rust binary
RUN cargo build --release

# Set the entrypoint
ENTRYPOINT ["./target/release/fibbot"]
