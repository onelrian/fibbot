FROM rust:latest as builder

# working directory
WORKDIR /app

# Copy the source code
COPY . .

# Building the application
RUN cargo build --release

# stage 2
FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# working directory
WORKDIR /app



COPY --from=builder /app/target/release/fibbot /usr/local/bin/fibbot


RUN chmod +x /usr/local/bin/fibbot


ENTRYPOINT ["fibbot"]

CMD ["true","10"]   