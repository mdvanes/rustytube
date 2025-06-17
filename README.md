# RustyTube

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
