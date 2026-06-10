# Remaining Backend Gaps — Final State

All originally identified backend contract gaps have been addressed. The
following summarises the final disposition of each item.

## Completed (backend + frontend built)

- [x] Course schema — `description`, `topics`, `prerequisites` fields
- [x] Notification preferences — `GET/PUT /api/me/preferences`
- [x] User search — `?search=` on `GET /api/user`
- [x] Date-filtered reports — `from`/`to` on org reward dashboard
- [x] Notification list/read — `GET /api/me/notifications`, `PUT …/{id}/read`
- [x] Org invite by email — `POST /api/organizations/{id}/members`
- [x] Teacher content edit — `updateTeacherContent` helper
- [x] Member audit history — table + logging + `GET …/audit`
- [x] API error normalization — `api_error` helper module
- [x] Assessment schema + list — DB tables + `GET /api/courses/{id}/assessments`
- [x] Assessment submission/scoring — `POST …/assessments/{id}/submit`, auto-score
- [x] Member remove — `DELETE /api/organizations/{id}/users/{uid}`
- [x] Role assignment — `POST …/users/{uid}/roles`

## Deferred / Architecture Decisions

- [ ] **Aggregated learner dashboard** — the frontend already composes
  `fetchCurrentSession` + `fetchCourseCatalog` + `fetchRewardHistory` +
  `fetchLearnerWallet` into `/learn`. A single-endpoint backend dashboard
  would be redundant.

- [ ] **Searchable applicant picker** — `GET /api/user?search=` exists
  but requires `VIEW_USER` (platform permission). Org operators need a
  scoped search; this is an intentional security boundary.

- [ ] **Wallet lifecycle states** — deposit/retirement/insufficient-funds
  depends on blockchain/smart-contract integration. Out of scope for the
  current backend API layer.

- [ ] **Assessment attempt history & retry UI** — backend endpoints exist;
  frontend lesson integration is future UI work.

- [ ] **API error envelope rollout** — the `api_error` helper pattern is
  established. Rolling it out to every handler is incremental refactoring.
