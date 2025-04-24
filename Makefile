# # ----------------Commands----------------
#
# # change the 20 value in printf to adjust width
# # Use ' ## some comment' behind a command and it will be added to the help message automatically
help: ## Show this help message
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

dev: ## Start up dev server
	docker compose -f compose.dev.yaml up --build --remove-orphans

dev-d: ## Start up dev server - detached mode
	docker compose -f compose.dev.yaml up -d --build --remove-orphans

dev-down: ## Tear down dev server
	docker compose -f compose.dev.yaml down


prod-d: ## Start up prod server
	docker compose -f compose.yaml up -d --build --remove-orphans

prod: ## Start up prod server
	docker compose -f compose.yaml up --build --remove-orphans


prod-down: ## Tear down prod server
	docker compose -f compose.yaml down

build:
	cargo build

test:
	DATABASE_URL=$(shell grep '^TEST_DATABASE_URL=' .env | cut -d '=' -f2) \
	APP_PORT=$(shell grep '^TEST_APP_PORT=' .env | cut -d '=' -f2) \
	APP_HOST=$(shell grep '^TEST_APP_HOST=' .env | cut -d '=' -f2) \
	cargo test -- --show-output

clean:
	cargo clean
	rm -rf target/

run:
	cargo run

coverage:
	cargo tarpaulin --ignore-tests

audit:
	cargo audit

check:
	cargo check
	cargo clippy -- -D warnings
	cargo fmt -- --check

watch:
	cargo watch -x run

migrate:
	sea-orm-cli migrate up

migrate-test:
	DATABASE_URL=$(shell grep '^TEST_DATABASE_URL=' .env | cut -d '=' -f2) \
	sea-orm-cli migrate up
