# Build stage
FROM rust:1.79 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin yarp
WORKDIR /yarp

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/yarp*
RUN cargo build --release

# Final stage
FROM ubuntu:focal

LABEL maintainer="Michael Riffle <mriffle@uw.edu>"
LABEL version="0.0.4"
LABEL description="Docker image for the yarp program"
LABEL org.opencontainers.image.source="https://github.com/mriffle/yarp"

#RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /yarp/target/release/yarp /usr/local/bin/yarp
COPY entrypoint.sh /usr/local/bin/entrypoint.sh

RUN chmod +x /usr/local/bin/entrypoint.sh && chmod +x /usr/local/bin/yarp

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
