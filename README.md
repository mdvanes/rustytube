# RustyTube

Realtime Public Transit visualisation. An implementation of TubeStar in Rust/egui with open data. 

## Run with Docker

Create a docker-compose.yml (the correct tag can be retrieved from https://github.com/mdvanes/rustytube/pkgs/container/rustytube):

```yaml
services:
  rustytube:
    image: docker pull ghcr.io/mdvanes/rustytube:sha-bea8c0a
    ports:
      - "8081:8081"
    environment:
      - PORT=8081
      - SOCKET_ADDRESS=0.0.0.0
      - STATIC_DIR=/app/static
```

And run with `docker compose up`. The application should be available on http://localhost:8081.

## Development

Start the front-end dev server with [just](https://just.systems/man/en/):

```bash
just dev
```

or with

```bash
    cd client && trunk serve &
    cd server && cargo watch -w src -x 'run --manifest-path Cargo.toml'
```

