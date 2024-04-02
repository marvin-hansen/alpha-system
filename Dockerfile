############################
# This Dockerfile runs relative to the repo root.
# Therefore, copy it to the root folder before running.
############################

# Set Rust version as ARG 
# https://docs.docker.com/build/guide/build-args/
ARG RUST_VERSION=1.77

# Optimizing CI/CD pipelines in your Rust projects
# https://blog.logrocket.com/optimizing-ci-cd-pipelines-rust-projects/
FROM asia-northeast1-docker.pkg.dev/future-309012/image-repo/rust-build:${RUST_VERSION}-alpine as chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

# Set service name
ARG SERVICE_NAME=""
# Set default build target
ARG BUILD_TARGET="aarch64-unknown-linux-musl"

RUN echo "SERVICE_NAME set to: ${SERVICE_NAME}"
RUN echo "BUILD_TARGET set to: ${BUILD_TARGET}"

# Set workspace directory
WORKDIR /app

# Export environment variable to override default memory allocator
ENV LD_PRELOAD=/usr/lib/libmimalloc.so

# Cache build with cargo cache
# Note, cargo cook and cargo build must be executed from the same working directory.
# https://github.com/LukeMathWalker/cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy over the entire source code
COPY . ./

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
