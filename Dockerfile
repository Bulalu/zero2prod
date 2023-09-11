# Use the cargo-chef image to manage dependencies
FROM lukemathwalker/cargo-chef:latest-rust-1.71.0 as chef
WORKDIR /app

# Update package list and install lld
RUN apt update --fix-missing
RUN apt install -y lld

# Copy the downloaded llvm-11-dev package into the image
COPY ./llvm-11-dev_11.0.1-2_arm64.deb /tmp/llvm-11-dev_11.0.1-2_arm64.deb

# Install the llvm-11-dev package
RUN dpkg -i /tmp/llvm-11-dev_11.0.1-2_arm64.deb || (apt-get update && apt-get -f install -y)

# Remove the package file
RUN rm /tmp/llvm-11-dev_11.0.1-2_arm64.deb

# Update package list again and install clang
RUN apt update --fix-missing
RUN apt install -y clang

# Compute a lock-like file for our project
FROM chef as planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

# Build our project dependencies, not our application!
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the rest of the source code
COPY . .
ENV SQLX_OFFLINE true

# Build our project
RUN cargo build --release --bin zero2prod

# Prepare the runtime image
FROM debian:bullseye-slim AS runtime
WORKDIR /app

# Install runtime dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary and other required files
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration

# Set environment variable
ENV APP_ENVIRONMENT production

# Set the entry point
ENTRYPOINT ["./zero2prod"]
