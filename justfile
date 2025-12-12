import 'docker.just'
import? 'private.just'

image_name := "ghcr.io/rezi-labs/focus"

docker:
    docker compose up

run:
    cargo run

install:
    cargo install --path .

watch:
    cargo watch -x run

verify: lint test

test:
    cargo test

lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt
    cargo fix --allow-dirty --allow-staged
