.PHONY: help build run stop test test-compose clean docker-build docker-up docker-down setup health runtime-verify runtime-log-scan runtime-disk docker-prune-build-cache \
  k8s-build k8s-apply k8s-dev-secrets k8s-dev-apply k8s-dev-refresh k8s-dev-delete k8s-delete k8s-status k8s-logs k8s-forward \
  k8s-validate dev-build dev-deps dev-run dev-worker worker-build migrate migrate-redo \
  dev-refresh test-integration fmt clippy web-lint web-build web-lint-compose web-build-compose logs ps shell

# Variables
export PATH := /opt/homebrew/bin:/usr/local/bin:$(PATH)
PROJECT_NAME := rust-learn
K8S_NAMESPACE := rust-learn
K8S_BASE := k8s/base
K8S_DEV := k8s/overlays/dev
K8S_IMAGE_TAG_DEFAULT := dev-$(shell date +%Y%m%d%H%M%S)
K8S_IMAGE_TAG ?= $(K8S_IMAGE_TAG_DEFAULT)
K8S_APP_IMAGE := rust-app:$(K8S_IMAGE_TAG)
K8S_WORKER_IMAGE := rust-worker:$(K8S_IMAGE_TAG)
K8S_WEB_IMAGE := web:$(K8S_IMAGE_TAG)
DOCKER ?= $(shell command -v docker 2>/dev/null || printf /opt/homebrew/bin/docker)
DOCKER_COMPOSE ?= $(DOCKER) compose
COMPOSE_REFRESH_SERVICES ?= app web
KUBECTL ?= $(shell command -v kubectl 2>/dev/null || printf /opt/homebrew/bin/kubectl)
MINIKUBE ?= $(shell command -v minikube 2>/dev/null || printf /opt/homebrew/bin/minikube)
CURL ?= $(shell command -v curl 2>/dev/null || printf curl)
HOST_CARGO ?= ./scripts/run-host-tests.sh
LOG_SCAN_SINCE ?= 30m
LOG_SCAN_PATTERN := level=(ERROR|WARN)|panic|traceback|unhandled|HTTP[[:space:]]+500|status=500|(^|[^[:alnum:]_=])500($|[^[:alnum:]_])

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m # No Color

help: ## Display this help message
	@echo "$(GREEN)Rust-Learn - Makefile$(NC)"
	@echo ""
	@echo "Available commands:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2}'

# Docker Compose
build: ## Build all Docker images
	$(DOCKER_COMPOSE) build

run: ## Start all services with docker-compose
	$(DOCKER_COMPOSE) up

dev: ## Start in detached mode (background)
	$(DOCKER_COMPOSE) up -d

dev-refresh: ## Rebuild selected app/web images and restart them without rebuilding dependencies
	$(DOCKER_COMPOSE) up -d --no-deps --build $(COMPOSE_REFRESH_SERVICES)

stop: ## Stop all services
	$(DOCKER_COMPOSE) down

docker-build: ## Build images for Kubernetes
	$(DOCKER) build -t rust-app:latest .
	$(DOCKER) build -t rust-worker:latest -f docker/worker.Dockerfile .
	$(DOCKER) build -t web:latest ./web

docker-up: ## Start with docker-compose in background
	$(DOCKER_COMPOSE) up -d

docker-down: ## Stop docker-compose
	$(DOCKER_COMPOSE) down

# Kubernetes
k8s-build: ## Build Docker images for Kubernetes
	@echo "$(YELLOW)Building Docker images...$(NC)"
	@set -e; \
	if command -v $(MINIKUBE) >/dev/null 2>&1 && [ "$$($(KUBECTL) config current-context 2>/dev/null)" = "minikube" ]; then \
		echo "$(YELLOW)Detected minikube context; building images directly inside minikube...$(NC)"; \
		$(MINIKUBE) image build -t rust-app:latest .; \
		$(MINIKUBE) image build -t rust-worker:latest -f docker/worker.Dockerfile .; \
		$(MINIKUBE) image build -t web:latest ./web; \
	else \
		$(DOCKER) build -t rust-app:latest .; \
		$(DOCKER) build -t rust-worker:latest -f docker/worker.Dockerfile .; \
		$(DOCKER) build -t web:latest ./web; \
	fi

k8s-apply: ## Apply all Kubernetes resources
	@echo "$(YELLOW)Applying Kubernetes resources...$(NC)"
	$(KUBECTL) apply -k $(K8S_BASE)/
	@echo "$(GREEN)Waiting for deployments to roll out...$(NC)"
	$(KUBECTL) rollout status deployment/postgres -n $(K8S_NAMESPACE) --timeout=180s
	$(KUBECTL) rollout status deployment/rustfs -n $(K8S_NAMESPACE) --timeout=180s
	$(KUBECTL) rollout status deployment/anvil -n $(K8S_NAMESPACE) --timeout=180s
	$(KUBECTL) rollout status deployment/rust-app -n $(K8S_NAMESPACE) --timeout=300s
	$(KUBECTL) rollout status deployment/worker -n $(K8S_NAMESPACE) --timeout=240s
	$(KUBECTL) rollout status deployment/web -n $(K8S_NAMESPACE) --timeout=180s
	@echo "$(GREEN)Deployment complete!$(NC)"

k8s-dev-secrets: ## Generate ignored local Kubernetes development secrets
	@./scripts/generate-k8s-dev-secrets.sh

k8s-dev-apply: k8s-dev-secrets ## Apply local Kubernetes overlay with generated development secrets
	@echo "$(YELLOW)Applying local Kubernetes development overlay...$(NC)"
	$(KUBECTL) apply -k $(K8S_DEV)/
	@echo "$(YELLOW)Restarting deployments that consume local secrets/config...$(NC)"
	$(KUBECTL) rollout restart deployment/anvil deployment/rustfs deployment/rust-app deployment/worker -n $(K8S_NAMESPACE)
	@echo "$(GREEN)Waiting for deployments to roll out...$(NC)"
	$(KUBECTL) rollout status deployment/postgres -n $(K8S_NAMESPACE) --timeout=180s
	$(KUBECTL) rollout status deployment/rustfs -n $(K8S_NAMESPACE) --timeout=180s
	$(KUBECTL) rollout status deployment/anvil -n $(K8S_NAMESPACE) --timeout=180s
	$(KUBECTL) rollout status deployment/rust-app -n $(K8S_NAMESPACE) --timeout=300s
	$(KUBECTL) rollout status deployment/worker -n $(K8S_NAMESPACE) --timeout=240s
	$(KUBECTL) rollout status deployment/web -n $(K8S_NAMESPACE) --timeout=180s
	@echo "$(GREEN)Development deployment complete!$(NC)"

k8s-dev-refresh: k8s-dev-secrets ## Rebuild, load, and redeploy local Kubernetes dev images with fresh tags
	@echo "$(YELLOW)Building fresh Kubernetes development images: $(K8S_APP_IMAGE), $(K8S_WORKER_IMAGE), $(K8S_WEB_IMAGE)...$(NC)"
	@set -e; \
	if command -v $(MINIKUBE) >/dev/null 2>&1 && [ "$$($(KUBECTL) config current-context 2>/dev/null)" = "minikube" ]; then \
		echo "$(YELLOW)Detected minikube context; building images directly inside minikube...$(NC)"; \
		$(MINIKUBE) image build -t $(K8S_APP_IMAGE) .; \
		$(MINIKUBE) image tag $(K8S_APP_IMAGE) rust-app:latest; \
		$(MINIKUBE) image build -t $(K8S_WORKER_IMAGE) -f docker/worker.Dockerfile .; \
		$(MINIKUBE) image tag $(K8S_WORKER_IMAGE) rust-worker:latest; \
		$(MINIKUBE) image build -t $(K8S_WEB_IMAGE) ./web; \
		$(MINIKUBE) image tag $(K8S_WEB_IMAGE) web:latest; \
	else \
		$(DOCKER) build -t rust-app:latest -t $(K8S_APP_IMAGE) .; \
		$(DOCKER) build -t rust-worker:latest -t $(K8S_WORKER_IMAGE) -f docker/worker.Dockerfile .; \
		$(DOCKER) build -t web:latest -t $(K8S_WEB_IMAGE) ./web; \
	fi
	@echo "$(YELLOW)Applying local Kubernetes development overlay...$(NC)"
	$(KUBECTL) apply -k $(K8S_DEV)/
	@echo "$(YELLOW)Restarting runtime dependencies that consume local secrets/config...$(NC)"
	$(KUBECTL) rollout restart deployment/anvil deployment/rustfs -n $(K8S_NAMESPACE)
	@echo "$(YELLOW)Pointing deployments at fresh image tags...$(NC)"
	$(KUBECTL) set image deployment/rust-app rust-app=$(K8S_APP_IMAGE) migrate=$(K8S_APP_IMAGE) wait-for-runtime-services=$(K8S_APP_IMAGE) -n $(K8S_NAMESPACE)
	$(KUBECTL) set image deployment/worker worker=$(K8S_WORKER_IMAGE) -n $(K8S_NAMESPACE)
	$(KUBECTL) set image deployment/web web=$(K8S_WEB_IMAGE) -n $(K8S_NAMESPACE)
	@echo "$(GREEN)Waiting for deployments to roll out...$(NC)"
	$(KUBECTL) rollout status deployment/postgres -n $(K8S_NAMESPACE) --timeout=180s
	$(KUBECTL) rollout status deployment/rustfs -n $(K8S_NAMESPACE) --timeout=180s
	$(KUBECTL) rollout status deployment/anvil -n $(K8S_NAMESPACE) --timeout=180s
	$(KUBECTL) rollout status deployment/rust-app -n $(K8S_NAMESPACE) --timeout=300s
	$(KUBECTL) rollout status deployment/worker -n $(K8S_NAMESPACE) --timeout=240s
	$(KUBECTL) rollout status deployment/web -n $(K8S_NAMESPACE) --timeout=180s
	@echo "$(GREEN)Development deployment refreshed with $(K8S_IMAGE_TAG)!$(NC)"

k8s-dev-delete: ## Delete local Kubernetes development overlay resources
	@echo "$(YELLOW)Deleting local Kubernetes development overlay...$(NC)"
	$(KUBECTL) delete -k $(K8S_DEV)/

k8s-delete: ## Delete all Kubernetes resources
	@echo "$(YELLOW)Deleting Kubernetes resources...$(NC)"
	$(KUBECTL) delete -k $(K8S_BASE)/
	@echo "$(GREEN)All resources have been deleted!$(NC)"

k8s-status: ## Display pod status
	@echo "$(GREEN)=== Pod Status ===$(NC)"
	$(KUBECTL) get pods -n $(K8S_NAMESPACE)
	@echo ""
	@echo "$(GREEN)=== Services ===$(NC)"
	$(KUBECTL) get svc -n $(K8S_NAMESPACE)
	@echo ""
	@echo "$(GREEN)=== Persistent Volume Claims ===$(NC)"
	$(KUBECTL) get pvc -n $(K8S_NAMESPACE)

k8s-logs: ## Display logs (use: make k8s-logs SERVICE=app)
	@if [ -z "$(SERVICE)" ]; then \
		echo "$(YELLOW)Usage: make k8s-logs SERVICE=service-name$(NC)"; \
		echo "Available services: postgres, rustfs, anvil, rust-app, worker, web"; \
		exit 1; \
	fi
	$(KUBECTL) logs -n $(K8S_NAMESPACE) -l app=$(SERVICE) --tail=100 -f

k8s-forward: ## Start port-forward (use: make k8s-forward SERVICE=web PORT=3000)
	@if [ -z "$(SERVICE)" ]; then \
		echo "$(YELLOW)Usage: make k8s-forward SERVICE=service-name PORT=port$(NC)"; \
		echo "Examples:"; \
		echo "  make k8s-forward SERVICE=web PORT=3000"; \
		echo "  make k8s-forward SERVICE=rust-app PORT=8080"; \
		echo "  make k8s-forward SERVICE=postgres PORT=5432"; \
		exit 1; \
	fi
	$(KUBECTL) port-forward -n $(K8S_NAMESPACE) svc/$(SERVICE) $(PORT):$(PORT)

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
	$(DOCKER_COMPOSE) up -d db rustfs anvil
	$(DOCKER_COMPOSE) --profile test run --rm test-runner cargo test $(CARGO_TEST_ARGS)

test-integration: ## Run integration tests
	$(DOCKER_COMPOSE) up -d anvil
	$(DOCKER_COMPOSE) --profile test run --rm --no-deps test-runner cargo test --test blockchain_integration_tests -- --ignored

fmt: ## Check Rust formatting
	cargo fmt --all --check

clippy: ## Run Rust Clippy on all targets and features
	$(HOST_CARGO) cargo clippy --all-targets --features app-bin,worker-bin,tool-bin -- -D warnings

k8s-validate: ## Render Kubernetes manifests locally
	$(KUBECTL) kustomize $(K8S_BASE) >/dev/null

web-lint: ## Run frontend lint checks
	cd web && npm run lint

web-build: ## Build the frontend
	cd web && npm run build

web-lint-compose: ## Run frontend lint checks inside the Docker Compose web service
	$(DOCKER_COMPOSE) run --rm --no-deps web sh -c 'npm ci --no-audit --no-fund && npm run lint'

web-build-compose: ## Build the frontend Docker image through Docker Compose
	$(DOCKER_COMPOSE) build web

# DB Migrations
migrate: ## Run Diesel migrations
	diesel migration run

migrate-redo: ## Redo last migration
	diesel migration redo

# Cleanup
clean: ## Delete generated files
	cargo clean
	$(DOCKER_COMPOSE) down -v --remove-orphans

# Utilities
logs: ## Display docker-compose logs (use: make logs SERVICE=app)
	@if [ -z "$(SERVICE)" ]; then \
		$(DOCKER_COMPOSE) logs -f; \
	else \
		$(DOCKER_COMPOSE) logs -f $(SERVICE); \
	fi

ps: ## Display running containers
	$(DOCKER_COMPOSE) ps

shell: ## Enter container shell (use: make shell SERVICE=app)
	@if [ -z "$(SERVICE)" ]; then \
		echo "$(YELLOW)Usage: make shell SERVICE=service-name$(NC)"; \
		exit 1; \
	fi
	$(DOCKER_COMPOSE) exec $(SERVICE) /bin/sh

# Setup
setup: ## Initial setup - create .env file and local JWT keys
	@./scripts/setup-env.sh
	@echo "$(YELLOW)Edit .env and configure environment-specific values!$(NC)"

# Health check
health: ## Check services health status
	@echo "$(GREEN)=== Health Check ===$(NC)"
	@echo "Docker:"
	@$(DOCKER_COMPOSE) ps || echo "$(YELLOW)Docker compose is not running$(NC)"
	@echo ""
	@echo "Docker endpoints:"
	@$(CURL) -fsS http://localhost:3000/healthz || echo "$(YELLOW)Web healthz is not reachable on localhost:3000$(NC)"
	@echo ""
	@$(CURL) -fsS http://localhost:8080/health || echo "$(YELLOW)API health is not reachable on localhost:8080$(NC)"
	@echo ""
	@$(CURL) -fsS http://localhost:8080/ready || echo "$(YELLOW)API ready is not reachable on localhost:8080$(NC)"
	@echo ""
	@echo ""
	@echo "Kubernetes:"
	@$(KUBECTL) get pods -n $(K8S_NAMESPACE) || echo "$(YELLOW)Kubernetes is not configured$(NC)"
	@echo ""
	@echo "Kubernetes endpoints:"
	@$(KUBECTL) exec -n $(K8S_NAMESPACE) deploy/web -- sh -c 'wget -qO- http://127.0.0.1:3000/healthz && printf "\n" && wget -qO- http://rust-app:8080/ready && printf "\n"' || echo "$(YELLOW)Kubernetes web/API readiness is not reachable from the web pod$(NC)"

runtime-verify: ## Fail unless Docker Compose and Kubernetes runtime checks pass
	@set -e; \
	echo "$(GREEN)=== Runtime Verification ===$(NC)"; \
	echo "Checking Docker Compose services and endpoints..."; \
	$(DOCKER_COMPOSE) ps web app worker db rustfs anvil >/dev/null; \
	$(CURL) -fsS http://localhost:3000/healthz >/dev/null; \
	$(CURL) -fsS http://localhost:8080/health >/dev/null; \
	$(CURL) -fsS http://localhost:8080/ready >/dev/null; \
	$(DOCKER_COMPOSE) exec -T worker /usr/local/bin/worker-healthcheck >/dev/null; \
	echo "$(GREEN)Docker Compose runtime OK$(NC)"; \
	echo "Checking Kubernetes deployments, pods, and in-cluster endpoints..."; \
	$(KUBECTL) get namespace $(K8S_NAMESPACE) >/dev/null; \
	$(KUBECTL) wait --for=condition=Available deployment --all -n $(K8S_NAMESPACE) --timeout=180s >/dev/null; \
	$(KUBECTL) wait --for=condition=Ready pod --all -n $(K8S_NAMESPACE) --timeout=180s >/dev/null; \
	$(KUBECTL) exec -n $(K8S_NAMESPACE) deploy/web -- sh -c 'wget -qO- http://127.0.0.1:3000/healthz >/dev/null && wget -qO- http://rust-app:8080/ready >/dev/null'; \
	echo "$(GREEN)Kubernetes runtime OK$(NC)"

runtime-log-scan: ## Show recent warning/error log lines from Docker Compose and Kubernetes
	@set -e; \
	failed=0; \
	scan_logs() { \
		label="$$1"; shift; \
		output_file=$$(mktemp); \
		echo ""; \
		echo "$$label"; \
		if "$$@" >"$$output_file" 2>&1; then \
			if grep -E -i '$(LOG_SCAN_PATTERN)' "$$output_file"; then \
				:; \
			else \
				echo "No recent warning/error log lines"; \
			fi; \
		else \
			failed=1; \
			echo "$(YELLOW)Unable to read logs$(NC)"; \
			sed -n '1,12p' "$$output_file"; \
		fi; \
		rm -f "$$output_file"; \
	}; \
	echo "$(GREEN)=== Runtime Log Scan ($(LOG_SCAN_SINCE)) ===$(NC)"; \
	scan_logs "Docker Compose app/worker/web:" $(DOCKER_COMPOSE) logs --no-color --since $(LOG_SCAN_SINCE) app worker web; \
	scan_logs "Kubernetes rust-app:" $(KUBECTL) logs -n $(K8S_NAMESPACE) deploy/rust-app --since=$(LOG_SCAN_SINCE); \
	scan_logs "Kubernetes worker:" $(KUBECTL) logs -n $(K8S_NAMESPACE) deploy/worker --since=$(LOG_SCAN_SINCE); \
	scan_logs "Kubernetes web:" $(KUBECTL) logs -n $(K8S_NAMESPACE) deploy/web --since=$(LOG_SCAN_SINCE); \
	exit $$failed

runtime-disk: ## Show Docker and Minikube disk usage
	@echo "$(GREEN)=== Runtime Disk Usage ===$(NC)"
	@echo "Docker:"
	@$(DOCKER) system df || echo "$(YELLOW)Docker is not reachable$(NC)"
	@echo ""
	@echo "Minikube:"
	@if $(MINIKUBE) status >/dev/null 2>&1; then \
		$(MINIKUBE) ssh -- df -h /var /; \
	else \
		echo "$(YELLOW)Minikube is not running$(NC)"; \
	fi

docker-prune-build-cache: ## Prune Docker build cache without removing images or volumes
	$(DOCKER) builder prune -f
