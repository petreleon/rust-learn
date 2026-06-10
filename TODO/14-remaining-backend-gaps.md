# Remaining Backend Gaps

Genuinely buildable items not yet implemented. QA, self-criticism, and
blockchain-integration items are excluded — those live in their existing files.

## High Priority

- [ ] **Assessment APIs** — `02-backend-contracts.md:39`

- [x] **Notification list/read APIs** — `02-backend-contracts.md:40`
  `GET /api/me/notifications` and `PUT /api/me/notifications/{id}/read`.
  Frontend helpers (`fetchNotifications`, `markNotificationRead`)
  added to `web/src/lib/session.ts`.

- [ ] **API error envelope normalization** — `02-backend-contracts.md:138`

## Medium Priority

- [ ] **Pagination metadata on all list/queue endpoints** — `02-backend-contracts.md:140`

- [x] **Organization invite by email** — `09-milestone-04-organization.md:33`
  `POST /api/organizations/{id}/members` with `{ email, role_name }`.
  Frontend `InviteMemberForm` component on the member directory page.

- [ ] **Member audit history** — `09-milestone-04-organization.md:33`

- [x] **Organization reward reports — date filters** — `09-milestone-04-organization.md:58`
  `from` / `to` date query params on
  `GET /api/reports/organizations/{orgId}/reward-dashboard` and `.csv`.
  Frontend date inputs in the organization reports route.

- [x] **Teacher content editing** — `08-milestone-03-teacher.md:151`
  `updateTeacherContent` helper added to `web/src/lib/teacher.ts`.
  Inline edit UI remains future frontend work.

## Low Priority

- [ ] **Searchable applicant picker endpoint** — `09-milestone-04-organization.md:55`

- [ ] **Wallet lifecycle response states** — `02-backend-contracts.md:150`

- [ ] **Aggregated learner dashboard endpoint** — `07-learner.md:43`

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
