# Use the minimal image
FROM rustlang/rust:nightly-slim AS build

# Where we will build the program
WORKDIR /src/slackify-wrapped

# Copy source code into the container
COPY . .

# Build the program in release mode
RUN cargo build --release

# Create the runtime image
FROM ubuntu:18.04

# Copy the compiled service binary
COPY --from=build /src/slackify-wrapped/target/release/slackify-wrapped /usr/local/bin/slackify-wrapped

# Start the service on container boot
CMD ["usr/local/bin/slackify-wrapped"]

