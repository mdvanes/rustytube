dev:
    cd client && trunk serve &
    cd server && cargo watch -w src -x 'run --manifest-path Cargo.toml'