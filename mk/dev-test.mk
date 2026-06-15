# Development
dev-build: ## Build only the Rust application (without container)
	cargo build --release --bin rust-learn --features app-bin

dev-deps: ## Start only API dependencies (Postgres, RustFS, Anvil)
	$(DOCKER_COMPOSE) up -d db rustfs anvil

dev-run: ## Run application locally (without container)
	cargo run --bin rust-learn --features app-bin

dev-worker: ## Run worker locally (without container)
	cargo run --bin worker --features worker-bin

worker-build: ## Build only the worker Docker image
	$(DOCKER_COMPOSE) build worker

# Tests
test: ## Run tests
	$(HOST_CARGO) cargo test $(CARGO_TEST_ARGS)

test-compose: ## Run tests through Docker Compose service networking
	$(DOCKER_COMPOSE) up -d --wait --wait-timeout 300 db rustfs anvil app web worker
	$(DOCKER_COMPOSE) --profile test run --rm -e RUN_DOCKER_COMPOSE_SMOKE=1 test-runner cargo test $(CARGO_TEST_ARGS)

test-integration: ## Run integration tests
	$(DOCKER_COMPOSE) up -d anvil
	$(DOCKER_COMPOSE) --profile test run --rm --no-deps test-runner cargo test --test blockchain_integration_tests -- --ignored

preflight: ## Run standard static, frontend, manifest, and runtime smoke checks
	$(MAKE) fmt
	$(MAKE) clippy
	$(MAKE) web-lint
	$(MAKE) web-api-helper-tests
	$(MAKE) web-build
	$(MAKE) k8s-validate
	$(MAKE) runtime-verify
	$(MAKE) runtime-log-scan

fmt: ## Check Rust formatting
	cargo fmt --all --check

fmt-compose: ## Check Rust formatting inside the Docker Compose test runner
	$(DOCKER_COMPOSE) --profile test run --rm --no-deps test-runner cargo fmt --all --check

clippy: ## Run Rust Clippy on all targets and features
	$(HOST_CARGO) cargo clippy --all-targets --features app-bin,worker-bin,tool-bin -- -D warnings

k8s-validate: k8s-dev-secrets ## Render base and local development Kubernetes manifests
	$(KUBECTL) kustomize $(K8S_BASE) >/dev/null
	$(KUBECTL) kustomize $(K8S_DEV) >/dev/null

k8s-dev-validate: k8s-dev-secrets ## Render local Kubernetes development overlay
	$(KUBECTL) kustomize $(K8S_DEV) >/dev/null

web-lint: ## Run frontend lint checks
	cd web && npm run lint

web-build: ## Build the frontend
	cd web && npm run build

web-api-helper-tests: ## Run frontend API helper contract tests
	cd web && npm run test:api-helpers

web-lint-compose: ## Run frontend lint checks inside the Docker Compose web service
	$(DOCKER_COMPOSE) run --rm --no-deps web sh -c 'npm ci --no-audit --no-fund && npm run lint'

web-build-compose: ## Build the frontend Docker image through Docker Compose
	$(DOCKER_COMPOSE) build web

mock-email: ## Preview local mock email output (use: make mock-email MOCK_EMAIL=... MOCK_NAME='Demo Learner')
	$(HOST_CARGO) cargo run --bin mock_email --features tool-bin -- "$(MOCK_EMAIL)" "$(MOCK_NAME)" "$(MOCK_TOKEN)"

# DB Migrations
diesel-compose: ## Run Diesel CLI through Compose (use: make diesel-compose DIESEL_ARGS='migration list')
	$(DOCKER_COMPOSE) up -d db
	$(DIESEL_COMPOSE) $(DIESEL_ARGS)

migration-generate: ## Generate a Diesel migration through Compose (use: make migration-generate NAME=create_table)
	@if [ -z "$(NAME)" ]; then \
		echo "$(YELLOW)Usage: make migration-generate NAME=create_table$(NC)"; \
		exit 1; \
	fi
	$(DIESEL_COMPOSE) migration generate $(NAME)

migrate: ## Run Diesel migrations through Docker Compose
	$(MAKE) diesel-compose DIESEL_ARGS='migration run'

migrate-redo: ## Redo last migration through Docker Compose
	$(MAKE) diesel-compose DIESEL_ARGS='migration redo'
