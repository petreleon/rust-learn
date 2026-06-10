# Remaining Backend Gaps

## Completed

- [x] **Notification list/read APIs** — `GET /api/me/notifications`, `PUT /api/me/notifications/{id}/read`
- [x] **Organization invite by email** — `POST /api/organizations/{id}/members` with `{ email, role_name }`
- [x] **Organization reward reports — date filters** — `from`/`to` params on reward dashboard
- [x] **Teacher content editing** — `updateTeacherContent` helper (PUT endpoint already existed)
- [x] **Course schema** — `description`, `topics`, `prerequisites` fields added
- [x] **Notification preferences** — `GET/PUT /api/me/preferences` + editable settings UI
- [x] **User search** — `?search=` param on `GET /api/user`
- [x] **Member audit history** — table + audit logging + `GET /api/organizations/{id}/members/{uid}/audit`
- [x] **API error normalization** — `api_error` helper module with consistent JSON envelope
- [x] **Assessment schema** — DB tables (`assessments`, `assessment_questions`, `assessment_attempts`) + list endpoint + frontend helper

## Remaining (substantial features)

- [ ] **Assessment submission/scoring** — submit answers, auto-score, retry logic (schema done, business logic pending)
- [ ] **API error envelope rollout** — replace plain-text errors across all endpoints with the new `api_error` helpers
- [ ] **Wallet lifecycle states** — deposit/retirement/insufficient-funds tracking (blockchain integration)
- [ ] **Searchable applicant picker** — org-scoped user search for nomination forms
- [ ] **Aggregated learner dashboard** — single endpoint composing enrollments, progress, rewards, wallet
