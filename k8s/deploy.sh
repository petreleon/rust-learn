#!/bin/bash
set -e

echo "================================"
echo "Rust-Learn Kubernetes Deployment"
echo "================================"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if kubectl is installed
if ! command -v kubectl &> /dev/null; then
    echo -e "${RED}Error: kubectl is not installed${NC}"
    exit 1
fi

# Check if kustomize is available
if ! kubectl kustomize --help &> /dev/null; then
    echo -e "${YELLOW}Warning: kustomize is not available via kubectl${NC}"
fi

# Functions
apply_base() {
    echo -e "${GREEN}Applying base Kubernetes resources...${NC}"
    kubectl apply -k k8s/base/
}

wait_for_pods() {
    echo ""
    echo -e "${GREEN}Waiting for pods to be ready...${NC}"
    echo ""
    
    echo "Waiting for PostgreSQL..."
    kubectl wait --for=condition=ready pod -l app=postgres -n rust-learn --timeout=180s || echo -e "${YELLOW}PostgreSQL timeout, continuing...${NC}"
    
    echo "Waiting for RustFS..."
    kubectl wait --for=condition=ready pod -l app=rustfs -n rust-learn --timeout=180s || echo -e "${YELLOW}RustFS timeout, continuing...${NC}"
    
    echo "Waiting for Anvil..."
    kubectl wait --for=condition=ready pod -l app=anvil -n rust-learn --timeout=180s || echo -e "${YELLOW}Anvil timeout, continuing...${NC}"
    
    echo "Waiting for App..."
    kubectl wait --for=condition=ready pod -l app=rust-app -n rust-learn --timeout=180s || echo -e "${YELLOW}App timeout, continuing...${NC}"
    
    echo "Waiting for Web..."
    kubectl wait --for=condition=ready pod -l app=web -n rust-learn --timeout=180s || echo -e "${YELLOW}Web timeout, continuing...${NC}"
}

show_status() {
    echo ""
    echo -e "${GREEN}=== Deployment Status ===${NC}"
    echo ""
    kubectl get pods -n rust-learn
    echo ""
    kubectl get svc -n rust-learn
    echo ""
    kubectl get pvc -n rust-learn
}

show_access_info() {
    echo ""
    echo -e "${GREEN}=== Access Information ===${NC}"
    echo ""
    echo "Services deployed:"
    echo "  - PostgreSQL:  postgres:5432"
    echo "  - RustFS S3:   rustfs:9000"
    echo "  - RustFS UI:   rustfs:9001"
    echo "  - Anvil:       anvil:8545"
    echo "  - App API:     rust-app:8080 (NodePort: 30080)"
    echo "  - Web Frontend: web:3000 (NodePort: 30030)"
    echo ""
    echo "To access services:"
    echo "  kubectl port-forward -n rust-learn svc/rust-app 8080:8080"
    echo "  kubectl port-forward -n rust-learn svc/web 3000:3000"
    echo "  kubectl port-forward -n rust-learn svc/rustfs 9001:9001"
    echo ""
}

case "${1:-apply}" in
    apply)
        apply_base
        wait_for_pods
        show_status
        show_access_info
        ;;
    status)
        show_status
        show_access_info
        ;;
    delete)
        echo -e "${YELLOW}Deleting all resources...${NC}"
        kubectl delete -k k8s/base/
        echo -e "${GREEN}All resources deleted${NC}"
        ;;
    rebuild)
        echo -e "${YELLOW}Rebuilding Docker images...${NC}"
        docker build -t rust-app:latest .
        docker build -t web:latest ./web
        echo -e "${GREEN}Images rebuilt successfully${NC}"
        ;;
    logs)
        if [ -z "$2" ]; then
            echo "Usage: $0 logs <app|worker|postgres|rustfs|anvil|web>"
            exit 1
        fi
        kubectl logs -n rust-learn -l app=$2 --tail=100 -f
        ;;
    exec)
        if [ -z "$2" ]; then
            echo "Usage: $0 exec <pod-name> [command]"
            exit 1
        fi
        kubectl exec -it -n rust-learn $2 -- ${3:-/bin/sh}
        ;;
    *)
        echo "Usage: $0 {apply|status|delete|rebuild|logs|exec}"
        echo ""
        echo "Commands:"
        echo "  apply   - Apply all Kubernetes resources (default)"
        echo "  status  - Show deployment status"
        echo "  delete  - Delete all resources"
        echo "  rebuild - Rebuild Docker images"
        echo "  logs    - View logs for a specific service"
        echo "  exec    - Execute a command in a pod"
        exit 1
        ;;
esac
