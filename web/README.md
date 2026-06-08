# RustLearn Web Console

Next.js operations console for reward, teacher-application, reporting, fraud,
delegation, and wallet workflows.

`/session` is the first product-session surface. It calls `GET /api/me` and
renders the resolved user profile, platform permissions, organization scopes,
course scopes, and active delegated permissions for route-guard work.

## Local Development

From the repository root, start the API dependencies and run the API:

```bash
make dev-deps
./scripts/run-host-tests.sh cargo run --bin rust-learn --features app-bin
```

Then start the web app:

```bash
cd web
npm run dev
```

Open <http://localhost:3000>. The app uses `/api` in the browser by default.
Next proxies `/api/*` to `${API_URL}/api/*` and `/health` to `${API_URL}/health`.
When `API_URL` is not set, it defaults to `http://127.0.0.1:8080`.
The web process exposes `GET /healthz` for container and Kubernetes probes so
health checks do not render the full dashboard.

Container and Kubernetes deployments keep browser requests on `/api` and set
`API_URL` explicitly:

- Docker Compose: `http://app:8080`
- Kubernetes: `http://rust-app:8080`

## Session Storage

The current product-session helper stores the JWT in `sessionStorage` under
`rustlearn.session.jwt`. This keeps the token out of persistent local storage
and clears it when the browser session ends, while still allowing a refresh of
product routes during local, Docker Compose, and Kubernetes checks. The tradeoff
is that any successful same-origin script injection could read the token, so the
final login flow should move toward secure HTTP-only cookies when the backend
adds cookie issuance and CSRF protection. Until then, product routes should load
`GET /api/me`, handle `401`, `403`, and `404` explicitly, and clear the stored
token on session expiry or logout.

## Current Session Contract

`GET /api/me` returns the authenticated user profile plus resolved platform,
organization, course, and delegated-permission scopes. The frontend session
helper treats `effective_permissions` as the route-guard source of truth and
keeps `direct_permissions` and `delegated_permissions` visible for UX copy and
audit context.

Expected error responses:

| Case | Status | Code/body | Frontend handling |
| --- | --- | --- | --- |
| Missing bearer token | `401` | JSON `error.code=unauthorized` | Clear stored token and show sign-in state. |
| Invalid bearer token | `401` | Text `Invalid token` from JWT middleware | Normalize to an unauthorized session error. |
| Expired bearer token | `401` | Text `Token expired` from JWT middleware | Clear stored token and show session-expired copy. |
| Unverified email | `403` | JSON `error.code=unverified_email` | Keep token out of privileged routes and show verification next step. |
| Missing user row | `404` | JSON `error.code=missing_user` | Clear stored token and return to sign-in. |

## Checks

```bash
npm run lint
npm run build
```
