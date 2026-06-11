# Kubernetes
k8s-build: ## Build Docker images for Kubernetes
	@echo "$(YELLOW)Building Docker images...$(NC)"
	@set -e; \
	if command -v $(MINIKUBE) >/dev/null 2>&1 && [ "$$($(KUBECTL) config current-context 2>/dev/null)" = "minikube" ]; then \
		echo "$(YELLOW)Detected minikube context; building locally and loading images into minikube...$(NC)"; \
		$(DOCKER) build -t rust-app:latest .; \
		$(DOCKER) build -t rust-worker:latest -f docker/worker.Dockerfile .; \
		$(DOCKER) build -t web:latest ./web; \
		$(MINIKUBE) image load rust-app:latest --daemon; \
		$(MINIKUBE) image load rust-worker:latest --daemon; \
		$(MINIKUBE) image load web:latest --daemon; \
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

k8s-dev-apply: k8s-dev-secrets k8s-build ## Build images and apply local Kubernetes overlay with generated development secrets
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
		echo "$(YELLOW)Detected minikube context; building locally and loading images into minikube...$(NC)"; \
		$(DOCKER) build -t rust-app:latest -t $(K8S_APP_IMAGE) .; \
		$(DOCKER) build -t rust-worker:latest -t $(K8S_WORKER_IMAGE) -f docker/worker.Dockerfile .; \
		$(DOCKER) build -t web:latest -t $(K8S_WEB_IMAGE) ./web; \
		$(MINIKUBE) image load $(K8S_APP_IMAGE) --daemon; \
		$(MINIKUBE) image load rust-app:latest --daemon; \
		$(MINIKUBE) image load $(K8S_WORKER_IMAGE) --daemon; \
		$(MINIKUBE) image load rust-worker:latest --daemon; \
		$(MINIKUBE) image load $(K8S_WEB_IMAGE) --daemon; \
		$(MINIKUBE) image load web:latest --daemon; \
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

k8s-dev-refresh-app: ## Rebuild and redeploy only the local Kubernetes app image
	@echo "$(YELLOW)Building fresh Kubernetes app image: $(K8S_APP_IMAGE)...$(NC)"
	@set -e; \
	if command -v $(MINIKUBE) >/dev/null 2>&1 && [ "$$($(KUBECTL) config current-context 2>/dev/null)" = "minikube" ]; then \
		echo "$(YELLOW)Detected minikube context; building locally and loading app image into minikube...$(NC)"; \
		$(DOCKER) build -t rust-app:latest -t $(K8S_APP_IMAGE) .; \
		$(MINIKUBE) image load $(K8S_APP_IMAGE) --daemon; \
		$(MINIKUBE) image load rust-app:latest --daemon; \
	else \
		$(DOCKER) build -t rust-app:latest -t $(K8S_APP_IMAGE) .; \
	fi
	@echo "$(YELLOW)Pointing app deployment at $(K8S_APP_IMAGE)...$(NC)"
	$(KUBECTL) set image deployment/rust-app rust-app=$(K8S_APP_IMAGE) migrate=$(K8S_APP_IMAGE) wait-for-runtime-services=$(K8S_APP_IMAGE) -n $(K8S_NAMESPACE)
	$(KUBECTL) rollout status deployment/rust-app -n $(K8S_NAMESPACE) --timeout=300s
	@echo "$(GREEN)App deployment refreshed with $(K8S_IMAGE_TAG)!$(NC)"

k8s-dev-refresh-web: ## Rebuild and redeploy only the local Kubernetes web image
	@echo "$(YELLOW)Building fresh Kubernetes web image: $(K8S_WEB_IMAGE)...$(NC)"
	@set -e; \
	if command -v $(MINIKUBE) >/dev/null 2>&1 && [ "$$($(KUBECTL) config current-context 2>/dev/null)" = "minikube" ]; then \
		echo "$(YELLOW)Detected minikube context; building locally and loading web image into minikube...$(NC)"; \
		$(DOCKER) build -t web:latest -t $(K8S_WEB_IMAGE) ./web; \
		$(MINIKUBE) image load $(K8S_WEB_IMAGE) --daemon; \
		$(MINIKUBE) image load web:latest --daemon; \
	else \
		$(DOCKER) build -t web:latest -t $(K8S_WEB_IMAGE) ./web; \
	fi
	@echo "$(YELLOW)Pointing web deployment at $(K8S_WEB_IMAGE)...$(NC)"
	$(KUBECTL) set image deployment/web web=$(K8S_WEB_IMAGE) -n $(K8S_NAMESPACE)
	$(KUBECTL) rollout status deployment/web -n $(K8S_NAMESPACE) --timeout=180s
	@echo "$(GREEN)Web deployment refreshed with $(K8S_IMAGE_TAG)!$(NC)"

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

k8s-forward: ## Start port-forward (use: make k8s-forward SERVICE=web PORT=3000 LOCAL_PORT=33030)
	@if [ -z "$(SERVICE)" ] || [ -z "$(PORT)" ]; then \
		echo "$(YELLOW)Usage: make k8s-forward SERVICE=service-name PORT=port [LOCAL_PORT=local-port]$(NC)"; \
		echo "Examples:"; \
		echo "  make k8s-forward SERVICE=web PORT=3000"; \
		echo "  make k8s-forward SERVICE=web PORT=3000 LOCAL_PORT=33030"; \
		echo "  make k8s-forward SERVICE=rust-app PORT=8080"; \
		echo "  make k8s-forward SERVICE=postgres PORT=5432"; \
		exit 1; \
	fi
	$(KUBECTL) port-forward -n $(K8S_NAMESPACE) svc/$(SERVICE) $(LOCAL_PORT):$(PORT)
