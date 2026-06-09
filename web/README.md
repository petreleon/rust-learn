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

Organization product routes now include `/organizations`,
`/organizations/[organizationId]`, and
`/organizations/[organizationId]/members`, and
`/organizations/[organizationId]/courses`, and
`/organizations/[organizationId]/reports`, and
`/organizations/[organizationId]/teacher-applications`. `/organizations` uses
the current session's organization scopes to show multi-organization workspace
cards, role-only memberships, delegated organization access, search,
capability filtering, summary counts, signed-out state, and no-organization
denied state without asking users to type raw ids. The organization dashboard
route shows the selected organization, role labels, direct/delegated/effective
permission counts, permission-derived available action capabilities, missing
scoped permission explanations, enabled reports, courses, members, and teacher
nomination actions when matching scoped permissions are present, disabled
contract-pending action buttons for unsupported capabilities, and stale
organization handling. The courses route loads
`GET /api/organizations/{organizationId}/courses` and shows organization
sponsored course summaries with lifecycle and reward filters, teacher labels,
content readiness, enrollment pressure, reward queue pressure, reward-policy
visibility, permission chips, empty state, denied state, and retryable
backend-failure state. The members route loads
`GET /api/organizations/{organizationId}/members` and shows organization
members with search, role and permission filters, pagination, email/KYC
readiness, role labels, direct/delegated/effective permission summaries,
operator action readiness, empty state, denied state, and retryable
backend-failure state. The reports route loads
`GET /api/reports/organizations/{organizationId}/reward-dashboard`, exports
`GET /api/reports/organizations/{organizationId}/reward-dashboard.csv`, and
shows all-time reward volume, approved amount, sponsored teacher application
summary, report-scoped wallet balance, course reward volume, export state,
empty state, denied state, and retryable backend-failure state. The teacher
applications route loads
`GET /api/organizations/{organizationId}/teacher-applications` and shows
sponsored/requested application tracking with search, status filters,
pagination, applicant labels, requested scope context, portfolio links, audit
hints, submitted/decided rows, empty state, denied state, and retryable
backend-failure state. Date filters, report pagination, payout-failure
drill-downs, reconciliation rows, invite/role-change/removal/audit actions,
course editing/publishing, teacher nomination submission, wallet audit, budget
data, and organization health metrics remain explicit backend-contract work
before those routes become full operator workflows.

Teacher product routes now include `/teach`, `/teach/apply`,
`/teach/courses`, `/teach/courses/[courseId]`, and
`/teach/courses/[courseId]/content`, and
`/teach/courses/[courseId]/enrollments`, and
`/teach/courses/[courseId]/students`, and
`/teach/courses/[courseId]/rewards`. `/teach` loads `GET /api/me`,
`GET /api/courses/teaching`, and `GET /api/teacher-applications/me` to show
application status, teaching course health, lifecycle state, content readiness,
enrollment pressure, reward review pressure, and scoped action permissions.
`/teach/courses` uses the same teaching-course contract with title search and
lifecycle filtering and links to `/teach/courses/[courseId]` for the real
course workspace. The workspace loads `GET /api/courses/teaching/{courseId}`
and shows lifecycle, ownership, teacher roles, action permissions, structured
chapters/content, data-present flags, inherited content publication state,
latest processing status/error, roster pressure, and reward review pressure.
The content authoring route loads the same workspace detail, creates chapters
through `POST /api/courses/{courseId}/chapters`, creates text/article content
through `POST /api/courses/{courseId}/chapters/{chapterId}/contents`, refreshes
the outline after successful saves, and disables authoring controls when the
session lacks course content permission. The enrollment route loads
`GET /api/courses/teaching/{courseId}/enrollments`, filters join requests,
approves/waitlists/rejects requests through
`PUT /api/courses/{courseId}/join-requests/{requestId}/decision`, shows roster
access state, and removes learners with
`DELETE /api/courses/{courseId}/enrollments/{userId}` after a two-step
confirmation. The students route loads
`GET /api/courses/teaching/{courseId}/students` to show enrolled learners,
content totals, unsupported persisted lesson progress, latest enrollment state,
and reward-candidate evidence/counts. The rewards route loads
`GET /api/courses/teaching/{courseId}/students` for learner context plus
`GET /api/courses/{courseId}/reward-candidates` for the course-scoped queue,
filters candidates by status, submits teacher approval/rejection through
`PUT /api/courses/{courseId}/reward-candidates/{candidateId}/teacher-decision`,
shows stale `409` conflict refresh copy, and avoids raw learner/candidate id
display when learner context is unavailable. Teacher screens do not expose
platform amount-review controls.
`GET /api/courses/teaching` returns
`total/limit/offset/search/lifecycle_status`, organization labels, content
summaries, reward-policy summaries, roster counts, reward queue counts, and
permission booleans for the current teacher scope. `/teach/apply` loads
`GET /api/me` and `GET /api/teacher-applications/me`, submits through
`POST /api/teacher-applications`, preserves draft application text and
portfolio links in session storage, and renders new, submitted, needs-changes,
approved, rejected, duplicate-conflict, signed-out, and backend-failure states.
Organization and course scope choices come from the resolved session context so
teachers and applicants do not type raw internal ids. Media upload, upload
retry/progress, destructive content editing, persisted lesson-completion
tracking, reward candidate submission, and platform amount review remain future
product routes.

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
