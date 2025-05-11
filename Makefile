# # ----------------Commands----------------
#
# # change the 20 value in printf to adjust width
# # Use ' ## some comment' behind a command and it will be added to the help message automatically
help: ## Show this help message
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

down: ## Tear down dev server
	docker compose -f compose.yaml --profile dev --profile test --profile prod down

up-dev: ## Start up dev server
	docker compose -f compose.yaml --profile dev up --build --remove-orphans

up-dev-d: ## Start up dev server - detached mode
	docker compose -f compose.yaml --profile dev up -d --build --remove-orphans

up-test: ## Start up dev server
	docker compose -f compose.yaml --profile test up --build --remove-orphans

up-test-d: ## Start up dev server - detached mode
	docker compose -f compose.yaml --profile test up -d --build --remove-orphans

up-prod-d: ## Start up prod server
	docker compose -f compose.yaml --profile prod up -d --build --remove-orphans

up-prod: ## Start up prod server
	docker compose -f compose.yaml --profile prod up --build --remove-orphans

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
	# cargo run
	bacon run

coverage:
	cargo tarpaulin --ignore-tests

audit:
	cargo audit

check:
	cargo check
	cargo clippy -- -D warnings
	cargo fmt -- --check

watch:
	# cargo watch -x run
	bacon run-long

watch-test:
	bacon test

migrate:
	sea-orm-cli migrate up

migrate-refresh:
	sea-orm-cli migrate refresh

migrate-test:
	DATABASE_URL=$(shell grep '^TEST_DATABASE_URL=' .env | cut -d '=' -f2) \
	sea-orm-cli migrate up
