# Local Kubernetes Overlay

This overlay is for local Minikube-style development. It patches the placeholder
base secrets with generated local values, removes the base ingress, and enables
RustFS's local single-disk bypass for one-PVC development clusters.

Generate the ignored secret patch before applying:

```bash
make k8s-dev-secrets
make k8s-dev-apply
```

`make k8s-dev-apply` also builds the local `latest` app, worker, and web images
before applying the overlay, so Kubernetes does not roll out pods that reference
missing local images. For faster code iteration after the overlay is already
running, use `make k8s-dev-refresh`, `make k8s-dev-refresh-app`, or
`make k8s-dev-refresh-web`.

The generated `secrets.patch.yaml` contains local development credentials and
must not be committed. The generator restricts it to the current user with
`0600` permissions. Existing secrets are preserved on later runs; set
`K8S_DEV_SECRETS_FORCE=1` to regenerate them.

The local Kubernetes admin bootstrap values come from the generated
`app-secrets` secret. They can differ from the repository-root `.env` used by
Docker Compose, so use the cluster secret values when validating authenticated
Kubernetes web flows.

Use the base manifests or a production overlay for clusters with a configured
ingress controller and production object-storage disks/PVs.
