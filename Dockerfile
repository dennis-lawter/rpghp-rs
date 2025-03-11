# Build image
FROM rust:latest AS build

WORKDIR /usr/src/app

# Copy in the code and config
COPY . .

# Build the application
RUN cargo build --release

# Runtime image
FROM debian:stable-slim

WORKDIR /usr/local/bin/rpghp-rs

# Copy the application and sqlx-cli from the build image
COPY --from=build /usr/src/app/target/release/rpghp-rs .
COPY --from=build /usr/src/app/templates ./templates
COPY --from=build /usr/src/app/migrations ./migrations
COPY --from=build /usr/local/cargo/bin/sqlx .

CMD ["./rpghp-rs"]
