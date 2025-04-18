########################################
# Build image
FROM rustlang/rust:nightly AS build

# ARG DATABASE_URL
ARG SQLX_OFFLINE

WORKDIR /usr/src/app

COPY . .

RUN ls -la

RUN rm .env
RUN mv .env.docker .env

RUN cargo build --release

########################################
# Runtime image
FROM debian:stable-slim

WORKDIR /usr/local/bin/rpghp-rs

# Copy the application from the build image
COPY --from=build /usr/src/app/target/release/rpghp-rs .

# Copy all the dependent files
COPY --from=build /usr/src/app/handlebars ./handlebars
COPY --from=build /usr/src/app/migrations ./migrations
COPY --from=build /usr/src/app/assets ./assets
COPY --from=build /usr/src/app/.env .

CMD ["./rpghp-rs"]
