# Builder stage
FROM rust:1.72.0 AS builder
# The `app` folder will be created by Docker in case it does not existe already.
WORKDIR /app
# Install the required system dependencies for the linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from the working environment to the Docker image
Copy . .
# Set SQLX_OFFILINE to force sqlx to look at the saved metadata instead of trying
# to query a live database
ENV SQLX_OFFLINE true
# Build the binary
# Using the release profile
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment
# to the runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# we need the configuration file at runtime!
COPY configuration configuration

# Use production configurations
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./zero2prod"]
