# Docker Compose
build: ## Build all Docker images
	$(DOCKER_COMPOSE) build

run: ## Start all services with Docker Compose
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

docker-up: ## Start with Docker Compose in background
	$(DOCKER_COMPOSE) up -d

docker-down: ## Stop Docker Compose
	$(DOCKER_COMPOSE) down
