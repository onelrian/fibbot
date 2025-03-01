# Stage 1: Build the Rust binary
FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/ticket /tickets

ENTRYPOINT ["/tickets"]
