FROM rust:1.72.0
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
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/zero2prod"]
