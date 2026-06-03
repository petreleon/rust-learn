.PHONY: help build run stop test clean docker-build docker-up docker-down setup health \
  k8s-build k8s-apply k8s-dev-secrets k8s-dev-apply k8s-dev-delete k8s-delete k8s-status k8s-logs k8s-forward \
  dev-build dev-deps dev-run dev-worker worker-build migrate migrate-redo \
  test-integration fmt web-lint web-build

# Variables
PROJECT_NAME := rust-learn
K8S_NAMESPACE := rust-learn
K8S_BASE := k8s/base
K8S_DEV := k8s/overlays/dev

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
	docker-compose build

run: ## Start all services with docker-compose
	docker-compose up

dev: ## Start in detached mode (background)
	docker-compose up -d

stop: ## Stop all services
	docker-compose down

docker-build: ## Build images for Kubernetes
	docker build -t rust-app:latest .
	docker build -t web:latest ./web

docker-up: ## Start with docker-compose in background
	docker-compose up -d

docker-down: ## Stop docker-compose
	docker-compose down

# Kubernetes
k8s-build: ## Build Docker images for Kubernetes
	@echo "$(YELLOW)Building Docker images...$(NC)"
	@if command -v minikube >/dev/null 2>&1 && [ "$$(kubectl config current-context 2>/dev/null)" = "minikube" ]; then \
		echo "$(YELLOW)Detected minikube context; building locally and loading images into minikube...$(NC)"; \
		docker build -t rust-app:latest .; \
		docker build -t web:latest ./web; \
		minikube image load rust-app:latest; \
		minikube image load web:latest; \
	else \
		docker build -t rust-app:latest .; \
		docker build -t web:latest ./web; \
	fi

k8s-apply: ## Apply all Kubernetes resources
	@echo "$(YELLOW)Applying Kubernetes resources...$(NC)"
	kubectl apply -k $(K8S_BASE)/
	@echo "$(GREEN)Waiting for deployments to roll out...$(NC)"
	kubectl rollout status deployment/postgres -n $(K8S_NAMESPACE) --timeout=180s
	kubectl rollout status deployment/rustfs -n $(K8S_NAMESPACE) --timeout=180s
	kubectl rollout status deployment/anvil -n $(K8S_NAMESPACE) --timeout=180s
	kubectl rollout status deployment/rust-app -n $(K8S_NAMESPACE) --timeout=300s
	kubectl rollout status deployment/worker -n $(K8S_NAMESPACE) --timeout=240s
	kubectl rollout status deployment/web -n $(K8S_NAMESPACE) --timeout=180s
	@echo "$(GREEN)Deployment complete!$(NC)"

k8s-dev-secrets: ## Generate ignored local Kubernetes development secrets
	@./scripts/generate-k8s-dev-secrets.sh

k8s-dev-apply: k8s-dev-secrets ## Apply local Kubernetes overlay with generated development secrets
	@echo "$(YELLOW)Applying local Kubernetes development overlay...$(NC)"
	kubectl apply -k $(K8S_DEV)/
	@echo "$(GREEN)Waiting for deployments to roll out...$(NC)"
	kubectl rollout status deployment/postgres -n $(K8S_NAMESPACE) --timeout=180s
	kubectl rollout status deployment/rustfs -n $(K8S_NAMESPACE) --timeout=180s
	kubectl rollout status deployment/anvil -n $(K8S_NAMESPACE) --timeout=180s
	kubectl rollout status deployment/rust-app -n $(K8S_NAMESPACE) --timeout=300s
	kubectl rollout status deployment/worker -n $(K8S_NAMESPACE) --timeout=240s
	kubectl rollout status deployment/web -n $(K8S_NAMESPACE) --timeout=180s
	@echo "$(GREEN)Development deployment complete!$(NC)"

k8s-dev-delete: ## Delete local Kubernetes development overlay resources
	@echo "$(YELLOW)Deleting local Kubernetes development overlay...$(NC)"
	kubectl delete -k $(K8S_DEV)/

k8s-delete: ## Delete all Kubernetes resources
	@echo "$(YELLOW)Deleting Kubernetes resources...$(NC)"
	kubectl delete -k $(K8S_BASE)/
	@echo "$(GREEN)All resources have been deleted!$(NC)"

k8s-status: ## Display pod status
	@echo "$(GREEN)=== Pod Status ===$(NC)"
	kubectl get pods -n $(K8S_NAMESPACE)
	@echo ""
	@echo "$(GREEN)=== Services ===$(NC)"
	kubectl get svc -n $(K8S_NAMESPACE)
	@echo ""
	@echo "$(GREEN)=== Persistent Volume Claims ===$(NC)"
	kubectl get pvc -n $(K8S_NAMESPACE)

k8s-logs: ## Display logs (use: make k8s-logs SERVICE=app)
	@if [ -z "$(SERVICE)" ]; then \
		echo "$(YELLOW)Usage: make k8s-logs SERVICE=service-name$(NC)"; \
		echo "Available services: postgres, rustfs, anvil, rust-app, worker, web"; \
		exit 1; \
	fi
	kubectl logs -n $(K8S_NAMESPACE) -l app=$(SERVICE) --tail=100 -f

k8s-forward: ## Start port-forward (use: make k8s-forward SERVICE=web PORT=3000)
	@if [ -z "$(SERVICE)" ]; then \
		echo "$(YELLOW)Usage: make k8s-forward SERVICE=service-name PORT=port$(NC)"; \
		echo "Examples:"; \
		echo "  make k8s-forward SERVICE=web PORT=3000"; \
		echo "  make k8s-forward SERVICE=rust-app PORT=8080"; \
		echo "  make k8s-forward SERVICE=postgres PORT=5432"; \
		exit 1; \
	fi
	kubectl port-forward -n $(K8S_NAMESPACE) svc/$(SERVICE) $(PORT):$(PORT)

# Development
dev-build: ## Build only the Rust application (without container)
	cargo build --release

dev-deps: ## Start only API dependencies (Postgres, RustFS, Anvil)
	docker-compose up -d db rustfs anvil

dev-run: ## Run application locally (without container)
	cargo run --bin rust-learn

dev-worker: ## Run worker locally (without container)
	cargo run --bin worker

worker-build: ## Build only the worker Docker image
	docker-compose build worker

# Tests
test: ## Run tests
	cargo test

test-integration: ## Run integration tests
	cargo test --test blockchain_integration_tests

fmt: ## Check Rust formatting
	cargo fmt --all --check

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
	docker-compose down -v --remove-orphans

# Utilities
logs: ## Display docker-compose logs (use: make logs SERVICE=app)
	@if [ -z "$(SERVICE)" ]; then \
		docker-compose logs -f; \
	else \
		docker-compose logs -f $(SERVICE); \
	fi

ps: ## Display running containers
	docker-compose ps

shell: ## Enter container shell (use: make shell SERVICE=app)
	@if [ -z "$(SERVICE)" ]; then \
		echo "$(YELLOW)Usage: make shell SERVICE=service-name$(NC)"; \
		exit 1; \
	fi
	docker-compose exec $(SERVICE) /bin/sh

# Setup
setup: ## Initial setup - create .env file and local JWT keys
	@./scripts/setup-env.sh
	@echo "$(YELLOW)Edit .env and configure environment-specific values!$(NC)"

# Health check
health: ## Check services health status
	@echo "$(GREEN)=== Health Check ===$(NC)"
	@echo "Docker:"
	@docker-compose ps || echo "$(YELLOW)Docker compose is not running$(NC)"
	@echo ""
	@echo "Kubernetes:"
	@kubectl get pods -n $(K8S_NAMESPACE) || echo "$(YELLOW)Kubernetes is not configured$(NC)"
