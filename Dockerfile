FROM rust:1.87-alpine3.21 as build-env

WORKDIR /app
RUN apk add --no-cache musl-dev
RUN rustup target add wasm32-unknown-unknown
# RUN cargo install wasm-bindgen-cli
RUN cargo install --locked wasm-bindgen-cli
RUN cargo install --locked trunk
COPY . .

WORKDIR /app/client
RUN trunk build

# RUN ls dist # remove TODO
# Next line not needed?
RUN cp -rf dist/* /app/server/src/static

WORKDIR /app/server
RUN cargo build --release

FROM alpine:3.21
WORKDIR /app
COPY --from=build-env /app/server/target/release/server /usr/local/bin/server
RUN mkdir -p /usr/local/bin/static
COPY --from=build-env /app/client/dist/* /usr/local/bin/static
ENTRYPOINT ["/usr/local/bin/server"]
