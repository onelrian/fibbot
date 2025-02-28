# Build stage
FROM rust:latest as builder

WORKDIR /app

# Install dependencies for musl-based static linking
RUN apt update && apt install -y musl-tools musl-dev libssl-dev pkg-config && rustup target add x86_64-unknown-linux-musl

# Set up OpenSSL for musl
RUN apt install -y build-essential
RUN wget https://www.openssl.org/source/openssl-1.1.1w.tar.gz && \
    tar -xzf openssl-1.1.1w.tar.gz && \
    cd openssl-1.1.1w && \
    ./Configure --prefix=/usr/local/musl --openssldir=/usr/local/musl/ssl no-shared no-zlib no-async linux-x86_64 && \
    make depend && \
    make -j$(nproc) && \
    make install

# Set environment variables for OpenSSL
ENV OPENSSL_DIR=/usr/local/musl
ENV OPENSSL_STATIC=1

# Copy Cargo files first for caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs  
RUN cargo build --release --target x86_64-unknown-linux-musl  

# Now copy actual source code
COPY src ./src

# Build the actual release binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# Rename binary for clarity
RUN mv target/x86_64-unknown-linux-musl/release/fibbot /app/fibbot

# Runtime stage
FROM alpine:latest

WORKDIR /app

# Copy compiled Rust binary
COPY --from=builder /app/fibbot /usr/local/bin/fibbot
RUN chmod +x /usr/local/bin/fibbot

# Run the program
CMD ["/usr/local/bin/fibbot"]