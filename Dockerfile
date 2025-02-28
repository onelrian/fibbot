# Use the official Rust image from Docker Hub
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Run the build command
RUN cargo build --release

# Check if the fibbot executable exists
RUN ls -l /app/target/release

# Make sure the fibbot executable is executable
RUN chmod +x /app/target/release/fibbot

# Set the entrypoint to run the fibbot executable
CMD ["/app/target/release/fibbot"]
