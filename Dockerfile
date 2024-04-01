############################
# This Dockerfile runs relative to the repo root.
# Therefore, copy it to the root folder before running.
############################

# Set Rust version
ARG RUST_VERSION=1.77

FROM asia-northeast1-docker.pkg.dev/future-309012/image-repo/rust-build:${RUST_VERSION}-alpine as builder

# Set service name
ARG SERVICE_NAME=""
# Set default build target
ARG BUILD_TARGET="aarch64-unknown-linux-musl"

RUN echo "SERVICE_NAME set to: ${SERVICE_NAME}"
RUN echo "BUILD_TARGET set to: ${BUILD_TARGET}"

# Set workspace directory
WORKDIR /app

# Copy over the entire source code
COPY . ./

# Fetch dependencies
RUN cargo fetch

# Export environment variable to override default memory allocator
ENV LD_PRELOAD=/usr/lib/libmimalloc.so

# Build the release binary.
RUN cargo build -p $SERVICE_NAME --release --target $BUILD_TARGET

# Move binary up to root level directory for easy access
RUN mv /app/target/$BUILD_TARGET/release/$SERVICE_NAME /service

# Strip the binary to reduce its size
RUN strip -s /service

############################
# Scratch image
############################
FROM asia-northeast1-docker.pkg.dev/future-309012/image-repo/rust-scratch:1.77-alpine as runner

# Copy binary from build output directory
COPY --from=builder /service /

# Port number must match deployment.yaml and specs/*/service_info
# service port: 7070
# metrics port: 8080
EXPOSE 7070 8080

# Run binary as unprivileged user
USER nobody:nobody

# Hard coded start command b/c no shell in scratch image
CMD ["/service"]
