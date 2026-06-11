.PHONY: help build run stop test test-compose clean docker-build docker-up docker-down setup health runtime-verify runtime-log-scan runtime-disk docker-prune-build-cache \
  k8s-build k8s-apply k8s-dev-secrets k8s-dev-apply k8s-dev-refresh k8s-dev-refresh-app k8s-dev-refresh-web k8s-dev-delete k8s-delete k8s-status k8s-logs k8s-forward \
  k8s-validate k8s-dev-validate dev-build dev-deps dev-run dev-worker worker-build migrate migrate-redo \
  dev-refresh test-integration preflight fmt clippy web-lint web-build web-api-helper-tests web-lint-compose web-build-compose logs ps shell

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
LOCAL_PORT ?= $(PORT)
LOG_SCAN_SINCE ?= 30m
LOG_SCAN_PATTERN := level=(ERROR|WARN)|panic|traceback|unhandled|HTTP[[:space:]]+500|status=500|(^|[^[:alnum:]_=])500($|[^[:alnum:]_])
WEB_DASHBOARD_SMOKE_TEXT ?= Opening your workspace

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m # No Color

help: ## Display this help message
	@echo "$(GREEN)Rust-Learn - Makefile$(NC)"
	@echo ""
	@echo "Available commands:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2}'

include mk/docker.mk
include mk/k8s.mk
include mk/dev-test.mk
include mk/ops.mk
