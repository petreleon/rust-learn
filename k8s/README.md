# Kubernetes Deployment for Rust-Learn

This directory contains Kubernetes manifests for running the Rust-Learn application.

## Structure

```
k8s/
├── base/                      # Base manifests
│   ├── namespace.yaml         # Application namespace
│   ├── secrets.yaml          # Secrets (needs editing)
│   ├── configmap.yaml        # Non-secret configurations
│   ├── postgres.yaml         # PostgreSQL with PVC
│   ├── rustfs.yaml           # RustFS (S3 storage) with PVC
│   ├── anvil.yaml            # Anvil (blockchain) with PVC
│   ├── app.yaml              # Rust API application
│   ├── worker.yaml           # Processing worker
│   ├── web.yaml              # Next.js frontend
│   ├── ingress.yaml          # Ingress for web access
│   └── kustomization.yaml    # Kustomize configuration
└── overlays/
    └── production/           # Production overlay
```

## Accessing the Web Application

There are **3 ways** to access the web application from Kubernetes:

### Method 1: Using Ingress (Recommended for Production)

**Prerequisites**: You need an Ingress Controller installed (e.g., NGINX Ingress Controller)

```bash
# Install NGINX Ingress Controller (if not already installed)
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/controller-v1.8.1/deploy/static/provider/cloud/deploy.yaml

# Wait for the ingress controller to be ready
kubectl wait --namespace ingress-nginx --for=condition=ready pod --selector=app.kubernetes.io/component=controller --timeout=90s
```

**Access the application:**

```bash
# Get the external IP/hostname
kubectl get ingress -n rust-learn

# If using Minikube
minikube tunnel

# Then access:
# http://localhost or the external IP from the ingress
```

**Features:**
- Single entry point for both web and API
- `/api/*` routes to the backend automatically
- Supports custom domain names
- Supports TLS/HTTPS

### Method 2: Using NodePort (Recommended for Local Development)

**Access the application:**

```bash
# Get the NodePort URL
kubectl get svc web-external -n rust-learn

# Using Minikube
minikube service web-external -n rust-learn

# Or access directly via node IP and port
# http://<node-ip>:30030
```

**Using the Makefile:**

```bash
# Port forward to localhost:3000
make k8s-forward SERVICE=web PORT=3000

# Or access via Minikube
minikube service web-external -n rust-learn --url
```

**Features:**
- Works without Ingress Controller
- Simple for local development
- Accessible on port 30030

### Method 3: Using LoadBalancer (Cloud Environments)

**Access the application:**

```bash
# Get the LoadBalancer IP
kubectl get svc web-lb -n rust-learn

# Wait for the EXTERNAL-IP to be assigned
watch kubectl get svc web-lb -n rust-learn

# Access via the external IP
# http://<external-ip>
```

**Features:**
- Best for cloud providers (AWS, GCP, Azure)
- Gets a public IP automatically
- Port 80 for easy access

## Pre-Deployment Configuration

### 1. Update Secrets

Edit `base/secrets.yaml` and update the values:
- PostgreSQL credentials (POSTGRES_USER, POSTGRES_PASSWORD, POSTGRES_DB)
- JWT keys (PRIVATE_KEY, PUBLIC_KEY)
- Ethereum mnemonic (ETH_MNEMONIC)
- Admin credentials (ADMIN_NAME, ADMIN_EMAIL, ADMIN_PASSWORD)
- MinIO credentials (S3_ACCESS_KEY, S3_SECRET_KEY)

### 2. Build Docker Images

```bash
# Build Rust application image
docker build -t rust-app:latest .

# Build web image
docker build -t web:latest ./web

# Or use the Makefile
make k8s-build
```

## Deployment

### Quick Deploy (using Makefile)

```bash
# Build images and deploy everything
make k8s-build
make k8s-apply
```

### Manual Deploy

```bash
# Create namespace
kubectl apply -f k8s/base/namespace.yaml

# Apply secrets and configmaps
kubectl apply -f k8s/base/secrets.yaml
kubectl apply -f k8s/base/configmap.yaml

# Apply base services (database, storage, blockchain)
kubectl apply -f k8s/base/postgres.yaml
kubectl apply -f k8s/base/rustfs.yaml
kubectl apply -f k8s/base/anvil.yaml

# Wait for base services to be ready
kubectl wait --for=condition=ready pod -l app=postgres --timeout=120s
kubectl wait --for=condition=ready pod -l app=rustfs --timeout=120s

# Apply application and worker
kubectl apply -f k8s/base/app.yaml
kubectl apply -f k8s/base/worker.yaml

# Apply frontend
kubectl apply -f k8s/base/web.yaml

# Apply ingress (optional)
kubectl apply -f k8s/base/ingress.yaml
```

### Using kubectl kustomize

```bash
# Apply all resources at once
kubectl apply -k k8s/base/
```

## Useful Commands

### Check Status

```bash
# View pod status
kubectl get pods -n rust-learn

# View services
kubectl get svc -n rust-learn

# View ingress
kubectl get ingress -n rust-learn

# Or use the Makefile
make k8s-status
```

### View Logs

```bash
# View application logs
kubectl logs -n rust-learn -l app=rust-app --tail=100 -f

# View web logs
kubectl logs -n rust-learn -l app=web --tail=100 -f

# Or use the Makefile
make k8s-logs SERVICE=web
```

### Port Forwarding

```bash
# Forward web to localhost:3000
kubectl port-forward -n rust-learn svc/web 3000:3000

# Forward API to localhost:8080
kubectl port-forward -n rust-learn svc/rust-app 8080:8080

# Or use the Makefile
make k8s-forward SERVICE=web PORT=3000
```

### Access Services

```bash
# Check application health
curl http://localhost:30030/health

# Check API directly
curl http://localhost:30080/health
```

## Troubleshooting

### Web page not loading

1. **Check if pods are running:**
   ```bash
   kubectl get pods -n rust-learn
   ```

2. **Check web logs:**
   ```bash
   kubectl logs -n rust-learn -l app=web
   ```

3. **Check if service is accessible:**
   ```bash
   kubectl get svc web-external -n rust-learn
   ```

4. **Test connectivity:**
   ```bash
   # From inside the cluster
   kubectl run test --rm -it --image=busybox --restart=Never -- wget -O- http://web:3000
   ```

### API not responding

1. **Check API pod status:**
   ```bash
   kubectl get pods -n rust-learn -l app=rust-app
   ```

2. **Check API logs:**
   ```bash
   kubectl logs -n rust-learn -l app=rust-app
   ```

3. **Check database connection:**
   ```bash
   kubectl logs -n rust-learn -l app=rust-app | grep -i database
   ```

### Ingress not working

1. **Check Ingress Controller:**
   ```bash
   kubectl get pods -n ingress-nginx
   ```

2. **Check Ingress rules:**
   ```bash
   kubectl get ingress -n rust-learn
   kubectl describe ingress rust-learn-ingress -n rust-learn
   ```

3. **Test without Ingress:**
   ```bash
   kubectl port-forward -n rust-learn svc/web 3000:3000
   # Open http://localhost:3000 in browser
   ```

## Monitoring

```bash
# View all resources
kubectl get all -n rust-learn

# View PersistentVolumeClaims
kubectl get pvc -n rust-learn

# View events
kubectl get events -n rust-learn --sort-by='.lastTimestamp'

# Watch pods
watch kubectl get pods -n rust-learn
```

## Security

⚠️ **Important**: In production:
1. Use external Kubernetes Secrets (sealed-secrets, vault, etc.)
2. Configure TLS/SSL for service exposure
3. Use NetworkPolicies
4. Periodic secret rotation
5. Enable RBAC
6. Use Pod Security Policies

## Customization

To customize the deployment for production, create an overlay in `overlays/production/`:

```bash
mkdir -p k8s/overlays/production
```

Example `kustomization.yaml` for production:

```yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

namespace: rust-learn-prod

resources:
  - ../../base

patchesStrategicMerge:
  - replicas.yaml
  - resources.yaml

configMapGenerator:
  - name: app-config
    behavior: merge
    literals:
      - PROD_MODE=TRUE
```

## Makefile Commands

```bash
# Deploy everything
make k8s-apply

# Delete deployment
make k8s-delete

# Check status
make k8s-status

# View logs
make k8s-logs SERVICE=web

# Port forward
make k8s-forward SERVICE=web PORT=3000
```

## Quick Start Guide

```bash
# 1. Build images
make k8s-build

# 2. Deploy
make k8s-apply

# 3. Access the application (choose one method)
# Option A: Port forward
make k8s-forward SERVICE=web PORT=3000

# Option B: Minikube
minikube service web-external -n rust-learn --url

# Option C: Check external IP
kubectl get svc -n rust-learn
```

## Architecture

```
                    ┌──────────────────────┐
                    │     Internet         │
                    └──────────┬───────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
       ┌──────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐
       │   Ingress   │ │  NodePort   │ │ LoadBalancer│
       │   :80/:443  │ │   :30030    │ │    :80      │
       └──────┬──────┘ └──────┬──────┘ └──────┬──────┘
              │               │               │
              └───────────────┼───────────────┘
                              │
                       ┌──────▼──────┐
                       │     Web     │
                       │   Port 3000 │
                       └──────┬──────┘
                              │
                    ┌─────────┴─────────┐
                    │                   │
            ┌───────▼───────┐  ┌───────▼───────┐
            │  Static Files │  │  /api/*       │
            │  (Frontend)   │  │  (Proxied)    │
            └───────────────┘  └───────┬───────┘
                                       │
                              ┌────────▼────────┐
                              │    Rust App     │
                              │   Port 8080     │
                              └────────┬────────┘
                                       │
       ┌──────────────┬────────────────┼────────────────┬──────────────┐
       │              │                │                │              │
  ┌────▼────┐   ┌─────▼─────┐   ┌─────▼─────┐   ┌──────▼──────┐ ┌────▼────┐
  │Postgres │   │   RustFS  │   │   Anvil   │   │   Worker    │ │  Redis  │
  │ :5432   │   │  :9000    │   │  :8545    │   │ (Processing)│ │(Future) │
  └─────────┘   └───────────┘   └───────────┘   └─────────────┘ └─────────┘
```
