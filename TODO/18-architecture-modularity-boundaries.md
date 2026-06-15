# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.

Objective: keep moving RustLearn from a technically layered monolith to a
Level 2 modular monolith with firm business boundaries. The goal is not more
folders; the goal is that HTTP, use cases, domain rules, adapters, permissions,
DTOs, and tests have clear owners.

## Target Shape

Use top-level architectural rings plus stable bounded-context folders. Do not
repeat `domain/application/infra/http` inside every context, and do not split
into multiple crates until Level 2 imports are clean enough to justify Level 3.

```text
src/
  bootstrap/       process wiring, app state, startup, route composition
  shared/          tiny cross-context primitives only
  domain/          pure business vocabulary, invariants, transitions
  application/     use cases, commands, outputs, errors, ports
  infra/           Postgres, object storage, Ethereum, email, dispatchers
  http/            Actix routes, extractors, DTOs, response mapping
```

Canonical contexts:

```text
identity, access_control, organizations, learning, content,
teacher_applications, kyc, rewards, wallet, reporting, notifications,
operations
```

Dependency rule:

```text
http -> application -> domain
infra -> application/domain ports
domain -> no Actix, Diesel, S3, Ethereum, env vars, or HTTP responses
bootstrap -> may know concrete wiring
```

`infra/postgres/<context>` should exist for every DB-owning context. Rewards is
only the deepest current extraction; it is not the only Postgres owner.

## Checked Facts

Live checks on 2026-06-15:

- No `src/api`, `src/services`, `src/repositories`, or `src/utils` files remain.
- No production `include!` calls remain under `src`.
- No `authenticated_user*` helper usage or direct `pool.get().await` remains in
  `src/http`.
- Remaining integration-test cleanup: `tests/video_upload_flow/imports.rs`.
- Remaining test `include!` calls: 4, all in `video_upload_flow`.

## Done Or Checked

| Area | Compact status |
| --- | --- |
| Bootstrap | `main.rs` is thin; app state, app data, startup checks, and route composition live under `bootstrap`. |
| Routes | Legacy `src/api` route wrappers are gone; `/api` is composed from context route configurators under `http`. |
| HTTP boundary | Migrated handlers mostly extract, call use cases, map DTOs, and return typed HTTP errors. JSON config/errors are centralized. |
| Application boundary | Migrated business workflows sit behind use-case commands, outputs, errors, and ports. |
| Domain boundary | Migrated domain code owns vocabulary, invariants, and transition helpers without Actix/Diesel/provider coupling. |
| Infra boundary | PostgreSQL, object storage, Ethereum, email, workers, and dispatchers are concrete adapters, not route owners. |
| Persistence | Async DB behavior was moved out of `models::*`; Diesel logic lives in `infra/postgres/<context>` or test fixtures. |
| Access control | Shared `AccessActor`, `AccessAction`, `AccessScope`, `can(...)`, and `can_any(...)` are used across middleware and migrated use cases. |
| Typed auth | Route handlers use typed auth extractors; `/api/me` keeps its JSON unauthorized response contract. |
| Frontend gates | Platform, organization, learner, teacher-application, and ops UI gates consume backend-derived current-session capabilities. |
| Test harnesses | Production source no longer uses `include!`; all but `video_upload_flow` integration harnesses now use explicit modules plus `support.rs`. |

## Verified Batches

Detailed history belongs in git; keep only proof that matters here.

| Batch | Commit | Proof |
| --- | --- | --- |
| 287 | `581f1be3` | Removed final request-auth helper; scanned away `authenticated_user*`, stray handler `HttpRequest` parsing, and direct `pool.get().await` in `src/http`. |
| 288-302 | `7cdddb44`..`8b3717e5` | Centralized JSON extractor errors and moved major context handlers to typed HTTP results/errors. |
| 303-312 | `19f31541`..`d0d0f32f` | Introduced typed access decisions and removed scope-specific permission wrappers from migrated use cases. |
| 313-315 | `8ac98ee2`..`4f8eb696` | Repointed frontend action gates to backend current-session capabilities and proved with frontend/backend checks. |
| 316-325 | `88308a68`..`46de0502` | Converted many integration harnesses from `include!`/`imports.rs` to explicit modules plus support helpers. |
| 326 | `4102c76a` | Converted `delegated_permissions`; focused tests, Cargo gates, boundary scans, and line checks passed. |
| 327 | `641ddd50` | Converted `reporting_exports`; fixed fixture idempotency keys and proved with focused tests plus standard gates. |
| 328 | `930d6586` | Converted `reward_candidates`; fixed idempotency keys and refreshed permission-envelope assertion. |
| 329 | `fc018ef7` | Converted `reward_execution`; focused reward-execution tests, Cargo gates, scans, and line checks passed. |
| 330 | this batch | Converted `wallet_linking`; refreshed the stale KYC conflict assertion to the typed HTTP error envelope and proved with focused wallet tests, Cargo gates, scans, and line checks. |

Standard proof set used for recent batches:

- `cargo fmt --all --check`
- focused host tests for touched contexts
- `./scripts/run-host-tests.sh cargo check --lib`
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`
- `git diff --check`
- touched-file size checks for manually maintained non-Markdown files
- boundary scans for legacy folders, production `include!`, HTTP DB-pool
  access, request-auth helper usage, and ring import leaks

## Current Context Snapshot

| Context | Checked Level 2 status |
| --- | --- |
| Access control | Role/delegation routes, delegated use cases, typed scopes/actions, shared decisions, middleware wiring, and session capabilities are in place. |
| Content | Chapter/content/upload/media/processing handlers use content use cases, Postgres adapters, typed errors, and one route configurator. |
| Identity | Session, profile/list, platform roles, login, verification, registration, reset, JWKS, and auth helper routes have ring ownership. |
| KYC | Status, submission, review, and audit flows have domain/application/Postgres/HTTP ownership. |
| Learning | Catalog, management, lifecycle, progress, roles, enrollment, assessment, and teaching routes use learning contracts/adapters/DTOs. |
| Notifications | Preferences and inbox flows live behind notification application/infra/http ownership; dispatch remains infra-owned. |
| Operations | Health/readiness route composition lives in `http/operations`; startup/readiness concerns live under operations/bootstrap/infra. |
| Organizations | CRUD, courses, members, audit, dashboard, invitations, roles, removal, and teacher tracking use organization contracts/adapters/DTOs. |
| Reporting | Platform summary, fraud/reward dashboards, wallet reconciliation, CSV exports, and organization reports have reporting ownership. |
| Rewards | Policy, fraud-block, history, audit, candidate, decision, compensation, payout, token, wallet-credit, notification, and reconciliation flows are migrated. |
| Teacher applications | Self-read, audit, list, platform review, submission, decision, nomination, and notification fan-out are migrated. |
| Wallet | Read/link/audit/token-tax/deposit/retirement/observed-deposit flows have application, Postgres, HTTP/worker ownership and access checks. |

## Remaining Work

- Convert `tests/video_upload_flow` from `include!`/`imports.rs` to explicit
  modules plus `support.rs`.
- Keep hardening authorization so middleware is an early rejection optimization
  and application use cases remain the real business guard.
- Keep hardening rewards around statuses/events/newtypes, candidate transitions,
  DTO leaks, Diesel adapter leaks, and fake-port use-case tests.
- Continue auditing data boundaries so Diesel schema/model leaks stay in infra
  or persistence records and public API DTOs stay HTTP-owned.
- Preserve route URLs and response semantics unless a migration note explicitly
  records a behavior change.

Suggested next batch: `tests/video_upload_flow`.

## Acceptance Criteria

- New routes can be added by touching one context plus shared contracts.
- Domain tests can run without Actix, Diesel, S3, Ethereum, env vars, or a DB
  pool.
- Use-case tests can run with fake ports for auth, persistence, notifications,
  and time.
- Permission behavior has one backend source of truth.
- New ring modules pass the import guardrails above.
- Public API DTOs and Diesel records are separate types unless a migration note
  accepts a temporary leak.
- No new module uses `include!` or `imports.rs`.
- Relevant format, host tests, Cargo checks, `git diff --check`, and boundary
  scans pass after each migration slice.

## Non-Goals

- Do not split into microservices yet.
- Do not split into Level 3 crates before Level 2 imports are clean.
- Do not rewrite all contexts at once.
- Do not add abstractions only to hide Diesel.
- Do not change product behavior during architecture migration unless a
  separate TODO item covers it.
