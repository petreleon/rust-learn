# RustLearn Web Console

Next.js operations console for reward, teacher-application, reporting, fraud,
delegation, and wallet workflows.

## Local Development

From the repository root, start the API dependencies and run the API:

```bash
make dev-deps
./scripts/run-host-tests.sh cargo run --bin rust-learn
```

Then start the web app:

```bash
cd web
npm run dev
```

Open <http://localhost:3000>. The app uses `/api` in the browser by default.
Next proxies `/api/*` to `${API_URL}/api/*` and `/health` to `${API_URL}/health`.
When `API_URL` is not set, it defaults to `http://127.0.0.1:8080`.

Container and Kubernetes deployments keep browser requests on `/api` and set
`API_URL` explicitly:

- Docker Compose: `http://app:8080`
- Kubernetes: `http://rust-app:8080`

## Checks

```bash
npm run lint
npm run build
```
