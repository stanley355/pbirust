# 1. This tells docker to use the Rust official image
FROM rust:1.56 as builder

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Migration
RUN cargo install diesel_cli --no-default-features --features postgres

# Build your program for release
RUN cargo build --release --all-features

# Run the binary
CMD ["./target/release/pbirust"]
