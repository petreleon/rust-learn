# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Verified base: `74811b12`; current checked batch listed below.

Goal: move RustLearn to a Level 2 modular monolith with firm business
boundaries. Ownership matters more than folder count. Preserve behavior unless
another TODO explicitly owns the behavior change.

## Level 2 Boundaries

- `bootstrap`: process wiring, app state, startup, route composition.
- `shared`: tiny cross-context primitives only.
- `domain`: pure vocabulary, invariants, transitions; no Actix, Diesel, S3,
  Ethereum, env vars, or HTTP responses.
- `application`: use cases, commands, outputs, errors, ports.
- `infra`: Postgres, object storage, Ethereum, email, workers, dispatchers.
- `http`: Actix routes, extractors, DTOs, response mapping.

Allowed flow: `http -> application -> domain`; `infra` implements
application/domain ports; `bootstrap` wires concrete dependencies.

Checked contexts: `access_control`, `content`, `identity`, `kyc`, `learning`,
`notifications`, `operations`, `organizations`, `reporting`, `rewards`,
`teacher_applications`, `wallet`.

## Already Done / Checked

- Thin bootstrap and route composition under `bootstrap`; no `src/api`.
- Migrated handlers use typed extractors, DTOs, typed errors, and application
  use cases.
- Migrated workflows use commands, outputs, ports, fake-port tests, access
  decisions, domain vocabulary, invariants, and transitions.
- Concrete adapters own Postgres, object storage, Ethereum, email, workers, and
  dispatchers; Diesel is kept out of route handlers and migrated
  application/domain code.
- Shared access-control vocabulary is used by middleware and use cases; typed
  auth extractors are in place; `/api/me` keeps its unauthorized JSON contract.
- Frontend gates consume backend current-session capabilities; tests no longer
  use `include!` or `imports.rs`.
- Scans checked: no legacy `src/{api,services,repositories,utils}`, no
  `include!`, no `tests/*/imports.rs`, no `authenticated_user*` or direct
  `pool.get().await` in `src/http`, and ring-import scans for `domain` and
  `application`.

## Verified Commit Groups

- `581f1be3`..`57215a3c`: auth helper removal, JSON extractor errors, typed
  HTTP results/errors, access decisions, frontend gates, test harness cleanup.
- `5a1643fa`..`2287a1f9`: reward status vocabulary at infra/application
  boundary for payout candidates, decisions, submissions, and reward history.
- `02388e62`: reward audit transition statuses and reconciliation final status.
- `986b1686`: reward source/event, audit/history/payout events, payment
  strategy, payout method.
- `b1413b5d`: reward policy scope/event/payment strategy, fraud-block
  scope/audit event, token-confirmation transaction type.
- `53a3fd67`: platform reward dashboard row vocabulary. Application
  outputs/facts carry typed reward event/status/execution status and
  reconciliation mismatch types; Postgres adapters parse persisted strings
  before crossing into application; HTTP DTOs keep the public string contract.
- `74811b12`: platform reward-approval CSV export vocabulary.
  Application facts/outputs carry typed reward source scope, event type, and
  candidate status; Postgres parses persisted strings before crossing into
  application; HTTP CSV serialization keeps the public string contract.
- Current verified batch: platform teacher-application CSV export vocabulary.
  Domain now exposes typed teacher-application scope/status enums while
  preserving existing normalization APIs; application export rows carry typed
  scope/status, Postgres parses persisted strings, and HTTP CSV serialization
  keeps the public string contract.

Proof set used for verified batches: focused host tests, fmt, Cargo lib/bin
checks, integration no-run compile, `git diff --check`, line-count checks,
reward/reporting string-field scans, and boundary scans.

## Still Open

- Rewards/reporting: remaining credit, notification event/payment, transition,
  CSV/export/reporting surfaces outside the verified dashboard rows and
  reward/teacher-application exports, wallet, and audit business strings
  crossing infra/application.
- Persistence: keep Diesel schema/model leakage inside infra/persistence
  records.
- HTTP: public API DTOs stay HTTP-owned, separate from Diesel records and
  application outputs.
- Authorization: use cases remain the real guard; middleware is early
  rejection.
- Constraints: no Level 3 crates/microservices, no all-at-once rewrite, no
  abstractions just to move files or hide Diesel.
