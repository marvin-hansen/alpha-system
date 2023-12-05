############################
# Build image
# This scripts runs relative to the repo root.
# Therefore, copy it there before running.
############################
FROM rust:1.74-slim-buster as builder

# Create the user and group files to run the container as an unprivileged user.
RUN mkdir /user && \
    echo 'nobody:x:65534:65534:nobody:/:' > /user/passwd && \
    echo 'nobody:x:65534:' > /user/group

# Install protobuf
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev

# Install build target tools
RUN rustup target add aarch64-unknown-linux-musl

# Set workspace directory
WORKDIR /app

# Copy over the entire source code
COPY . ./

# Fix linker issue with musel
# https://github.com/rust-lang/stacker/issues/80
ARG CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
ARG CC=aarch64-linux-gnu-gcc

# Fetch dependencies
RUN cargo fetch

# Run the actual build.
RUN cargo build -p dbgw --release --target aarch64-unknown-linux-musl

# Move binary up to root level directory for easy access
RUN mv /app/target/aarch64-unknown-linux-musl/release/dbgw /service

############################
# Scratch image
############################
# Create a minimal docker image
FROM scratch as runner

# Import user and group files from the build stage.
COPY --from=builder /user/group /user/passwd /etc/

# Import the CAcertificates to enable HTTPS.
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy binary from build output directory
COPY --from=builder /service /

# Port number must match deployment.yaml and specs/*/service_info
EXPOSE 6060 8080

# Run binary as unprivileged user
USER nobody:nobody

# Hard coded start command b/c no shell in scratch image
CMD ["/service"]