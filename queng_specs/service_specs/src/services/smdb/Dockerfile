############################
# This Dockerfile runs relative to the repo root.
# Therefore, copy it to the root folder before running.
############################
FROM ghcr.io/marvin-hansen/mimalloc/rust-build:1.77-alpine as build

# Set workspace directory
WORKDIR /app

# Copy over the entire source code
COPY . ./

# Fetch dependencies if they change.
RUN cargo fetch

# Build the release binary.
RUN cargo build -p smdb --release --target aarch64-unknown-linux-musl

# Move binary up to root level directory for easy access
RUN mv /app/target/aarch64-unknown-linux-musl/release/smdb /smdb

# Strip the binary to reduce its size
RUN strip -s /smdb

############################
# Scratch image
############################
FROM hansenmarvin/rust-scratch:latest as runner

# Copy binary from build output directory
COPY --from=build /smdb /

# Port number must match deployment.yaml and specs/*/service_info
# service port: 9090
# metrics port: 8080
EXPOSE 9090 8080

# Run binary as unprivileged user
USER nobody:nobody

# Hard coded start command b/c no shell in scratch image
CMD ["/smdb"]
