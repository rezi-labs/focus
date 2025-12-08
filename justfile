import 'docker.just'
import? 'private.just'

image_name := "ghcr.io/rezi-labs/taste"
export LOCAL := "true"

export ADMIN_USERNAME :='admin'
export ADMIN_PASSWORD :='admin'
export RESET_ADMIN_USER := 'false'

docker:
    docker compose up

run:
    cargo run

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

generate-session-secret:
    openssl rand -base64 64
