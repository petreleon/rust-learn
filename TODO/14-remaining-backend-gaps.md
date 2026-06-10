# Remaining Backend Gaps

Genuinely buildable items not yet implemented. QA, self-criticism, and
blockchain-integration items are excluded — those live in their existing files.

## High Priority

- [ ] **Assessment APIs** — `02-backend-contracts.md:39`
  Create/list assessments for a course, submit answers, score, return
  results, and handle retries. Needs DB tables (`assessments`,
  `assessment_questions`, `assessment_attempts`, `assessment_answers`),
  models, service, and API routes under `/api/courses/{id}/assessments`.
  Frontend deferred until these endpoints exist.

- [ ] **Notification list/read APIs** — `02-backend-contracts.md:40`
  `GET /api/notifications` (paginated, filtered by read/unread) and
  `PUT /api/notifications/{id}/read` (or `POST …/mark-all-read`). The
  backend already sends notifications; it needs a retrieval contract.

- [ ] **API error envelope normalization** — `02-backend-contracts.md:138`
  Replace the mix of JSON `{ "error": { … } }` and plain-text errors
  across every Actix handler with a single consistent JSON error shape.
  Affects every endpoint; a helper function or middleware should handle
  this.

## Medium Priority

- [ ] **Pagination metadata on all list/queue endpoints** — `02-backend-contracts.md:140`
  Ensure every list/report/audit endpoint returns `{ total, limit, offset }`
  so the frontend can build reusable pagination controls without guessing.

- [ ] **Organization invite by email** — `09-milestone-04-organization.md:33`
  `POST /api/organizations/{id}/invites` with `{ email }` body. Creates a
  pending invite record, sends email notification. Accept/expire logic
  with `GET /api/invites/{token}` and `POST /api/invites/{token}/accept`.

- [ ] **Member audit history** — `09-milestone-04-organization.md:33`
  `GET /api/organizations/{orgId}/members/{userId}/audit` returning
  role-change, permission-change, and removal events with timestamps
  and actor context.

- [ ] **Organization reward reports — date filters and pagination**
  — `09-milestone-04-organization.md:58`
  Add `from` / `to` date query params to the existing
  `GET /api/reports/organizations/{orgId}/reward-dashboard` and its
  `.csv` variant. Wire into the frontend reports route.

- [ ] **Teacher content editing/publishing** — `08-milestone-03-teacher.md:151`
  The frontend content authoring route currently handles create-only for
  chapters and text content. Editing content data, reordering, and
  publishing lifecycle changes need mutation endpoints (the
  `PUT /api/courses/{id}/chapters/{chId}/contents/{cId}` endpoint
  already exists for raw updates; the frontend just needs wiring).

## Low Priority

- [ ] **Searchable applicant picker endpoint** — `09-milestone-04-organization.md:55`
  A user-search endpoint accessible to organization operators (without
  `VIEW_USER` platform permission). Currently org operators can only
  nominate via raw user ID. Needs a scoped search or an org-member
  lookup.

- [ ] **Wallet lifecycle response states** — `02-backend-contracts.md:150`
  `GET /api/wallets/me` should return distinct statuses for
  linked, unlinked, pending-deposit, confirmed-deposit, retirement,
  and insufficient-funds. Depends on blockchain/contract integration.

- [ ] **Aggregated learner dashboard endpoint** — `07-learner.md:43`
  `GET /api/learner/dashboard` returning enrollments, progress, due
  work, notification summary, reward summary, and wallet summary in
  one call. The current `/learn` route composes these from separate
  requests.

## Already Adequate (no new backend work needed)

- **Reward history enrichment** — status labels, token tx, wallet credit,
  reconciliation indicators already returned by
  `GET /api/reward-candidates/me/history` and consumed by the frontend.
- **Wallet summary enrichment** — linked/unlinked/value states already
  returned by `GET /api/wallets/me`. Deposit/retirement states need
  blockchain integration (tracked above).
- **Fraud-block CRUD API** — admin routes already exercise create/list/
  revoke endpoints.
- **Delegated-permission CRUD API** — admin routes already exercise
  grant/list/revoke endpoints.
- **Organization dashboard summary** — `GET /api/organizations/{id}/dashboard`
  already returns members, courses, teacher applications, reward volume,
  wallet balance, alerts, and pending actions.
