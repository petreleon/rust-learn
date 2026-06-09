# RustLearn Web

Next.js product frontend and internal operations console for RustLearn.

`/` resolves the current session and routes users to the right product surface:
anonymous users go to `/login`, and users with a stored token are checked
against `GET /api/me` before going to `/session`. `/ops` is the internal
operations console for reward, teacher-application, reporting, fraud,
delegation, wallet, export, and local API-debugging workflows.

`/login` is the product sign-in route for email/password authentication.
`/register` creates unverified learner accounts and shows the local mock-email
next step. `/verify-email` confirms account email tokens before login.
`/session` is the first product-session surface. It uses the token created by
`/login`, calls `GET /api/me`, and renders the resolved user profile, platform
permissions, organization scopes, course scopes, and active delegated
permissions for route-guard work. Manual bearer-token entry belongs in `/ops`,
not in product routes.

`/settings/account` loads `GET /api/me` plus `GET /api/wallets/me` to show
profile details, email/KYC readiness, self-wallet linked/unlinked/error state,
workspace counts, and mobile-first next-step actions. Notification preferences
are shown as disabled defaults with explicit copy until the backend exposes
preference read/save endpoints.

The first permission-aware workspace routes are `/learn`, `/teach`,
`/organizations`, and `/admin`. They all load `GET /api/me`, derive navigation
from effective permissions instead of role labels, and show contextual denied
states when the current user lacks the required learner, teacher, organization,
or platform access signal.

Teacher product routes now include `/teach`, `/teach/apply`, and
`/teach/courses`. `/teach` loads `GET /api/me`,
`GET /api/courses/teaching`, and `GET /api/teacher-applications/me` to show
application status, teaching course health, lifecycle state, content readiness,
enrollment pressure, reward review pressure, and scoped action permissions.
`/teach/courses` uses the same teaching-course contract with title search and
lifecycle filtering. The route keeps course workspace, enrollment, and reward
review actions disabled until those deeper product routes exist, and it does
not expose teacher-side reward amount controls. `GET /api/courses/teaching`
returns `total/limit/offset/search/lifecycle_status`, organization labels,
content summaries, reward-policy summaries, roster counts, reward queue counts,
and permission booleans for the current teacher scope. `/teach/apply` loads
`GET /api/me` and `GET /api/teacher-applications/me`, submits through
`POST /api/teacher-applications`, preserves draft application text and
portfolio links in session storage, and renders new, submitted, needs-changes,
approved, rejected, duplicate-conflict, signed-out, and backend-failure states.
Organization and course scope choices come from the resolved session context so
teachers and applicants do not type raw internal ids. Content upload,
enrollment decision queues, student progress, and reward candidate decision
screens remain future product routes.

Learner product routes now include `/learn`, `/courses`, `/rewards`, and
`/wallet`. `/learn` is the learner dashboard. It loads `GET /api/me`, enrolled
and recommended `GET /api/courses/catalog` slices,
`GET /api/reward-candidates/me/history`, and `GET /api/wallets/me` in a
client-side aggregate so learners see enrolled courses, a continue-learning
action, reward status, wallet readiness, recommendations, signed-out state, and
first-run empty state. Persisted progress, due work, last activity, and
notifications stay visibly untracked until those backend contracts exist.
`/courses` loads `GET /api/courses/catalog`, supports title search, enrollment
filters, reward-only filtering, refresh, visible course cards, detail links,
and `POST /api/courses/{courseId}/join-requests` for learner enrollment
requests. `/courses/[courseId]` loads
`GET /api/courses/catalog/{courseId}` and shows organization, teacher,
content, syllabus, reward, enrollment, signed-out, and not-found states.
`/courses/[courseId]/learn` loads
`GET /api/courses/catalog/{courseId}/learn` and shows the course outline,
selected lesson, text content, next/previous navigation, safe non-text
not-rendered copy, media processing states, failed-processing errors,
content-denied errors, and signed-out state. The contract returns
`progress_supported=false`, so the current UI labels progress as local only
until persisted completion endpoints exist.
Authored course descriptions, topics, and prerequisites remain empty until the
backend course schema stores them. `/rewards` loads
`GET /api/reward-candidates/me/history`, supports human status filters, and
separates teacher review, amount approval, token processing, wallet credit,
reconciliation, and failed states. `/wallet` loads `GET /api/wallets/me` plus
recent `GET /api/reward-candidates/me/history` rows, treats
`404 Wallet not linked` as an empty state, puts the link action before metrics
for unlinked mobile learners, shows wallet-credit history without raw internal
wallet/user ids, and uses `POST /api/wallets/me/link` for the self-service link
action. Deposits and retirements stay explicitly unavailable in this UI until
their product contracts exist.

The shared product shell includes global status notices for session expiry and
retryable workspace failures. Its account and mobile menus show delegated
permissions with scope labels and expiration text so temporary access is visible
without exposing the internal operations console as primary product navigation.

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

Open <http://localhost:3000>. The product entry redirects anonymous users to
`/login`; open <http://localhost:3000/ops> for the internal workflow console.
The app uses `/api` in the browser by default.
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
npm run test:api-helpers
npm run build
```
