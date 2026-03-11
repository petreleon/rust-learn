.PHONY: help build run stop test clean docker-build docker-up docker-down setup health \
  k8s-build k8s-apply k8s-delete k8s-status k8s-logs k8s-forward \
  dev-build dev-run dev-worker migrate migrate-redo \
  test-integration

# Variables
PROJECT_NAME := rust-learn
K8S_NAMESPACE := rust-learn

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
	docker build -t rust-app:latest .
	docker build -t web:latest ./web

k8s-apply: ## Apply all Kubernetes resources
	@echo "$(YELLOW)Applying Kubernetes resources...$(NC)"
	kubectl apply -k k8s/base/
	@echo "$(GREEN)Waiting for pods to be ready...$(NC)"
	kubectl wait --for=condition=ready pod -l app=postgres -n $(K8S_NAMESPACE) --timeout=180s || true
	kubectl wait --for=condition=ready pod -l app=rustfs -n $(K8S_NAMESPACE) --timeout=180s || true
	kubectl wait --for=condition=ready pod -l app=anvil -n $(K8S_NAMESPACE) --timeout=180s || true
	kubectl wait --for=condition=ready pod -l app=rust-app -n $(K8S_NAMESPACE) --timeout=180s || true
	kubectl wait --for=condition=ready pod -l app=web -n $(K8S_NAMESPACE) --timeout=180s || true
	@echo "$(GREEN)Deployment complete!$(NC)"

k8s-delete: ## Delete all Kubernetes resources
	@echo "$(YELLOW)Deleting Kubernetes resources...$(NC)"
	kubectl delete -k k8s/base/
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

dev-run: ## Run application locally (without container)
	cargo run --bin rust-learn

dev-worker: ## Run worker locally (without container)
	cargo run --bin worker

# Tests
test: ## Run tests
	cargo test

test-integration: ## Run integration tests
	cargo test --test blockchain_integration_tests

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
setup: ## Initial setup - create .env file
	@if [ ! -f .env ]; then \
		cp .env.example .env; \
		echo "$(GREEN).env file created from .env.example$(NC)"; \
		echo "$(YELLOW)Edit .env and configure necessary values!$(NC)"; \
	else \
		echo "$(YELLOW).env file already exists$(NC)"; \
	fi

# Health check
health: ## Check services health status
	@echo "$(GREEN)=== Health Check ===$(NC)"
	@echo "Docker:"
	@docker-compose ps || echo "$(YELLOW)Docker compose is not running$(NC)"
	@echo ""
	@echo "Kubernetes:"
	@kubectl get pods -n $(K8S_NAMESPACE) || echo "$(YELLOW)Kubernetes is not configured$(NC)"
