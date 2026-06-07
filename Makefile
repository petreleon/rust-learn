.PHONY: help build run stop test test-compose clean docker-build docker-up docker-down setup health \
  k8s-build k8s-apply k8s-dev-secrets k8s-dev-apply k8s-dev-refresh k8s-dev-delete k8s-delete k8s-status k8s-logs k8s-forward \
  k8s-validate dev-build dev-deps dev-run dev-worker worker-build migrate migrate-redo \
  test-integration fmt web-lint web-build

# Variables
export PATH := /opt/homebrew/bin:/usr/local/bin:$(PATH)
PROJECT_NAME := rust-learn
K8S_NAMESPACE := rust-learn
K8S_BASE := k8s/base
K8S_DEV := k8s/overlays/dev
K8S_IMAGE_TAG ?= dev-$(shell date +%Y%m%d%H%M%S)
K8S_RUST_IMAGE := rust-app:$(K8S_IMAGE_TAG)
K8S_WEB_IMAGE := web:$(K8S_IMAGE_TAG)
DOCKER ?= $(shell command -v docker 2>/dev/null || printf /opt/homebrew/bin/docker)
DOCKER_COMPOSE ?= $(DOCKER) compose
KUBECTL ?= $(shell command -v kubectl 2>/dev/null || printf /opt/homebrew/bin/kubectl)
MINIKUBE ?= $(shell command -v minikube 2>/dev/null || printf /opt/homebrew/bin/minikube)

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m # No Color

help: ## Display this help message
	@echo "$(GREEN)Rust-Learn - Makefile$(NC)"
	@echo ""
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2}'

# Docker Compose
build: ## Build all Docker images
	$(DOCKER_COMPOSE) build

run: ## Start all services with docker-compose
	$(DOCKER_COMPOSE) up

dev: ## Start in detached mode (background)
	$(DOCKER_COMPOSE) up -d

stop: ## Stop all services
	$(DOCKER_COMPOSE) down

docker-build: ## Build images for Kubernetes
	$(DOCKER) build -t rust-app:latest .
	$(DOCKER) build -t web:latest ./web

docker-up: ## Start with docker-compose in background
	$(DOCKER_COMPOSE) up -d

docker-down: ## Stop docker-compose
	$(DOCKER_COMPOSE) down

# Kubernetes
k8s-build: ## Build Docker images for Kubernetes
	@echo "$(YELLOW)Building Docker images...$(NC)"
	@if command -v $(MINIKUBE) >/dev/null 2>&1 && [ "$$($(KUBECTL) config current-context 2>/dev/null)" = "minikube" ]; then \
		echo "$(YELLOW)Detected minikube context; building images directly inside minikube...$(NC)"; \
		$(MINIKUBE) image build -t rust-app:latest .; \
		$(MINIKUBE) image build -t web:latest ./web; \
	else \
		$(DOCKER) build -t rust-app:latest .; \
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
	@echo "$(YELLOW)Building fresh Kubernetes development images: $(K8S_RUST_IMAGE), $(K8S_WEB_IMAGE)...$(NC)"
	@if command -v $(MINIKUBE) >/dev/null 2>&1 && [ "$$($(KUBECTL) config current-context 2>/dev/null)" = "minikube" ]; then \
		echo "$(YELLOW)Detected minikube context; building images directly inside minikube...$(NC)"; \
		$(MINIKUBE) image build -t $(K8S_RUST_IMAGE) .; \
		$(MINIKUBE) image tag $(K8S_RUST_IMAGE) rust-app:latest; \
		$(MINIKUBE) image build -t $(K8S_WEB_IMAGE) ./web; \
		$(MINIKUBE) image tag $(K8S_WEB_IMAGE) web:latest; \
	else \
		$(DOCKER) build -t rust-app:latest -t $(K8S_RUST_IMAGE) .; \
		$(DOCKER) build -t web:latest -t $(K8S_WEB_IMAGE) ./web; \
	fi
	@echo "$(YELLOW)Applying local Kubernetes development overlay...$(NC)"
	$(KUBECTL) apply -k $(K8S_DEV)/
	@echo "$(YELLOW)Restarting runtime dependencies that consume local secrets/config...$(NC)"
	$(KUBECTL) rollout restart deployment/anvil deployment/rustfs -n $(K8S_NAMESPACE)
	@echo "$(YELLOW)Pointing deployments at fresh image tags...$(NC)"
	$(KUBECTL) set image deployment/rust-app rust-app=$(K8S_RUST_IMAGE) migrate=$(K8S_RUST_IMAGE) wait-for-runtime-services=$(K8S_RUST_IMAGE) -n $(K8S_NAMESPACE)
	$(KUBECTL) set image deployment/worker worker=$(K8S_RUST_IMAGE) -n $(K8S_NAMESPACE)
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
	cargo build --release

dev-deps: ## Start only API dependencies (Postgres, RustFS, Anvil)
	$(DOCKER_COMPOSE) up -d db rustfs anvil

dev-run: ## Run application locally (without container)
	cargo run --bin rust-learn

dev-worker: ## Run worker locally (without container)
	cargo run --bin worker

worker-build: ## Build only the worker Docker image
	$(DOCKER_COMPOSE) build worker

# Tests
test: ## Run tests
	./scripts/run-host-tests.sh

test-compose: ## Run tests through Docker Compose service networking
	$(DOCKER_COMPOSE) up -d db rustfs anvil
	$(DOCKER_COMPOSE) --profile test run --rm test-runner cargo test $(CARGO_TEST_ARGS)

test-integration: ## Run integration tests
	cargo test --test blockchain_integration_tests

fmt: ## Check Rust formatting
	cargo fmt --all --check

k8s-validate: ## Render Kubernetes manifests locally
	$(KUBECTL) kustomize $(K8S_BASE) >/dev/null

web-lint: ## Run frontend lint checks
	cd web && npm run lint

web-build: ## Build the frontend
	cd web && npm run build

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
	@echo "Kubernetes:"
	@$(KUBECTL) get pods -n $(K8S_NAMESPACE) || echo "$(YELLOW)Kubernetes is not configured$(NC)"
