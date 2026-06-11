# Cleanup
clean: ## Delete generated files
	cargo clean
	$(DOCKER_COMPOSE) down -v --remove-orphans

# Utilities
logs: ## Display Docker Compose logs (use: make logs SERVICE=app)
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
	$(CURL) -fsS http://localhost:3000 | grep -q '$(WEB_DASHBOARD_SMOKE_TEXT)'; \
	$(CURL) -fsS http://localhost:3000/health >/dev/null; \
	$(CURL) -fsS http://localhost:3000/ready >/dev/null; \
	$(DOCKER_COMPOSE) exec -T worker /usr/local/bin/worker-healthcheck >/dev/null; \
	echo "$(GREEN)Docker Compose runtime OK$(NC)"; \
	echo "Checking Kubernetes deployments, pods, and in-cluster endpoints..."; \
	$(KUBECTL) get namespace $(K8S_NAMESPACE) >/dev/null; \
	$(KUBECTL) wait --for=condition=Available deployment --all -n $(K8S_NAMESPACE) --timeout=180s >/dev/null; \
	$(KUBECTL) wait --for=condition=Ready pod --all -n $(K8S_NAMESPACE) --timeout=180s >/dev/null; \
	$(KUBECTL) exec -n $(K8S_NAMESPACE) deploy/web -- sh -c 'wget -qO- http://127.0.0.1:3000/healthz >/dev/null && wget -qO- http://127.0.0.1:3000 | grep -q "$(WEB_DASHBOARD_SMOKE_TEXT)" && wget -qO- http://127.0.0.1:3000/ready >/dev/null'; \
	echo "$(GREEN)Kubernetes runtime OK$(NC)"

runtime-log-scan: ## Fail on recent warning/error log lines from Docker Compose and Kubernetes
	@set -e; \
	failed=0; \
	scan_logs() { \
		label="$$1"; shift; \
		output_file=$$(mktemp); \
		echo ""; \
		echo "$$label"; \
		if "$$@" >"$$output_file" 2>&1; then \
			if grep -E -i '$(LOG_SCAN_PATTERN)' "$$output_file"; then \
				failed=1; \
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
