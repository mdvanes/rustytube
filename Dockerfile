FROM rust:1.87-alpine3.21 as build-env
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY . .
WORKDIR /app/server
RUN cargo build --release

FROM alpine:3.21
WORKDIR /app
COPY --from=build-env /app/server/target/release/server /usr/local/bin/server
ENTRYPOINT ["/usr/local/bin/server"]
