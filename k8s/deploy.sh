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

K8S_NAMESPACE="${K8S_NAMESPACE:-rust-learn}"

find_tool() {
    local name="$1"
    local candidate

    if command -v "$name" &> /dev/null; then
        command -v "$name"
        return 0
    fi

    for candidate in "/opt/homebrew/bin/$name" "/usr/local/bin/$name"; do
        if [ -x "$candidate" ]; then
            echo "$candidate"
            return 0
        fi
    done

    return 1
}

KUBECTL="${KUBECTL:-$(find_tool kubectl || true)}"
DOCKER="${DOCKER:-$(find_tool docker || true)}"

require_docker() {
    if [ -z "$DOCKER" ]; then
        echo -e "${RED}Error: docker is not installed${NC}"
        exit 1
    fi
}

# Check if kubectl is installed
if [ -z "$KUBECTL" ]; then
    echo -e "${RED}Error: kubectl is not installed${NC}"
    exit 1
fi

# Check if kustomize is available
if ! "$KUBECTL" kustomize --help &> /dev/null; then
    echo -e "${YELLOW}Warning: kustomize is not available via kubectl${NC}"
fi

# Functions
apply_base() {
    echo -e "${GREEN}Applying base Kubernetes resources...${NC}"
    "$KUBECTL" apply -k k8s/base/
}

wait_for_pods() {
    echo ""
    echo -e "${GREEN}Waiting for pods to be ready...${NC}"
    echo ""
    
    echo "Waiting for PostgreSQL..."
    "$KUBECTL" wait --for=condition=ready pod -l app=postgres -n "$K8S_NAMESPACE" --timeout=180s || echo -e "${YELLOW}PostgreSQL timeout, continuing...${NC}"
    
    echo "Waiting for RustFS..."
    "$KUBECTL" wait --for=condition=ready pod -l app=rustfs -n "$K8S_NAMESPACE" --timeout=180s || echo -e "${YELLOW}RustFS timeout, continuing...${NC}"
    
    echo "Waiting for Anvil..."
    "$KUBECTL" wait --for=condition=ready pod -l app=anvil -n "$K8S_NAMESPACE" --timeout=180s || echo -e "${YELLOW}Anvil timeout, continuing...${NC}"
    
    echo "Waiting for App..."
    "$KUBECTL" wait --for=condition=ready pod -l app=rust-app -n "$K8S_NAMESPACE" --timeout=180s || echo -e "${YELLOW}App timeout, continuing...${NC}"
    
    echo "Waiting for Web..."
    "$KUBECTL" wait --for=condition=ready pod -l app=web -n "$K8S_NAMESPACE" --timeout=180s || echo -e "${YELLOW}Web timeout, continuing...${NC}"
}

show_status() {
    echo ""
    echo -e "${GREEN}=== Deployment Status ===${NC}"
    echo ""
    "$KUBECTL" get pods -n "$K8S_NAMESPACE"
    echo ""
    "$KUBECTL" get svc -n "$K8S_NAMESPACE"
    echo ""
    "$KUBECTL" get pvc -n "$K8S_NAMESPACE"
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
    echo "  $KUBECTL port-forward -n $K8S_NAMESPACE svc/rust-app 8080:8080"
    echo "  $KUBECTL port-forward -n $K8S_NAMESPACE svc/web 3000:3000"
    echo "  $KUBECTL port-forward -n $K8S_NAMESPACE svc/rustfs 9001:9001"
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
        "$KUBECTL" delete -k k8s/base/
        echo -e "${GREEN}All resources deleted${NC}"
        ;;
    rebuild)
        require_docker
        echo -e "${YELLOW}Rebuilding Docker images...${NC}"
        "$DOCKER" build -t rust-app:latest .
        "$DOCKER" build -t rust-worker:latest -f docker/worker.Dockerfile .
        "$DOCKER" build -t web:latest ./web
        echo -e "${GREEN}Images rebuilt successfully${NC}"
        ;;
    logs)
        if [ -z "$2" ]; then
            echo "Usage: $0 logs <app|worker|postgres|rustfs|anvil|web>"
            exit 1
        fi
        "$KUBECTL" logs -n "$K8S_NAMESPACE" -l "app=$2" --tail=100 -f
        ;;
    exec)
        if [ -z "$2" ]; then
            echo "Usage: $0 exec <pod-name> [command]"
            exit 1
        fi
        pod_name="$2"
        shift 2
        if [ "$#" -eq 0 ]; then
            set -- /bin/sh
        fi
        "$KUBECTL" exec -it -n "$K8S_NAMESPACE" "$pod_name" -- "$@"
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
