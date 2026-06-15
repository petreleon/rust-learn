# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Verified base before this changeset: `f8212c71`.
Latest verified changeset: wallet owner-type vocabulary.

Goal: move RustLearn to a Level 2 modular monolith with firm business
boundaries. Ownership matters more than folder count; preserve behavior unless
another TODO owns the behavior change.

## Level 2 Boundaries

- `bootstrap`: process wiring, app state, startup, route composition.
- `shared`: tiny cross-context primitives only.
- `domain`: pure vocabulary, invariants, transitions; no Actix, Diesel, S3,
  Ethereum, env vars, or HTTP responses.
- `application`: use cases, commands, outputs, errors, ports.
- `infra`: Postgres, object storage, Ethereum, email, workers, dispatchers.
- `http`: Actix routes, extractors, DTOs, response mapping.

Allowed flow: `http -> application -> domain`; `infra` implements ports;
`bootstrap` wires concrete dependencies.

Checked contexts: `access_control`, `content`, `identity`, `kyc`, `learning`,
`notifications`, `operations`, `organizations`, `reporting`, `rewards`,
`teacher_applications`, `wallet`.

## Done / Checked

- Bootstrap is thin; migrated HTTP uses typed extractors, DTOs, errors, and use
  cases.
- Migrated workflows use commands, outputs, ports, fake-port tests, access
  decisions, domain vocabulary, invariants, and transitions.
- Infra owns Postgres, object storage, Ethereum, email, workers, dispatchers;
  Diesel is kept out of route handlers and migrated application/domain code.
- Shared access-control vocabulary, typed auth extractors, frontend capability
  gates, and `/api/me` unauthorized JSON contract are checked.
- Boundary scans checked: no legacy `src/{api,services,repositories,utils}`, no
  `include!`, no `tests/*/imports.rs`, no direct auth/pool access in `http`,
  and no forbidden ring imports in `domain`/`application`.

## Verified Commit Groups

- `581f1be3`..`57215a3c`: auth helper removal, JSON extractor errors, typed
  HTTP results/errors, access decisions, frontend gates, test harness cleanup.
- `5a1643fa`..`2287a1f9`: reward status vocabulary for payout candidates,
  decisions, submissions, and reward history.
- `02388e62`, `986b1686`, `b1413b5d`: reward audit/reconciliation/source/event,
  payment, policy, fraud, and token-confirmation vocabulary.
- `53a3fd67`: platform reward dashboard rows are typed across app/infra/http.
- `74811b12`: platform reward-approval CSV export vocabulary.
- `285bdfcb`: platform teacher-application CSV export vocabulary.
- `e5e3a578`: platform delegated-permission CSV export vocabulary.
- `f8212c71`: platform fraud-dashboard scope vocabulary.
- This changeset: wallet owner-type vocabulary for wallet read/audit and
  platform wallet reconciliation outputs.

Proof set for verified batches: focused host tests, fmt, Cargo lib/bin checks,
integration no-run compile, `git diff --check`, line counts, string-field scans,
and boundary scans. This changeset additionally passed wallet focused tests and
the platform wallet reconciliation reporting path.

## Still Open

- Rewards/reporting: remaining credit, notification event/payment, transition,
  CSV/export/reporting surfaces not listed above, wallet, and audit strings
  crossing infra/application.
- Persistence: keep Diesel schema/model leakage inside infra/persistence
  records.
- HTTP: public API DTOs stay HTTP-owned, separate from Diesel records and
  application outputs.
- Authorization: use cases remain the real guard; middleware is early
  rejection.
- Constraints: no Level 3 crates/microservices, no all-at-once rewrite, no
  abstractions just to move files or hide Diesel.
