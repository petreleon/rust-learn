# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.

Objective: keep moving RustLearn from a technically layered monolith to a
Level 2 modular monolith with firm business boundaries. The target is not more
files for their own sake; the target is that HTTP, use cases, domain rules,
adapters, permissions, DTOs, and tests have clear owners.

## Design Basis Checked

- Actix `App`, `Scope`, extractors, `web::Data`, and `ResponseError` docs.
- Rust module system and Rust API Guidelines.
- Existing project shape, route wiring, application use cases, Postgres
  adapters, middleware, frontend permission gates, and integration-test
  harnesses.

Decision after self-criticism: use top-level architectural rings, then stable
bounded-context folders, then use-case or aggregate folders where a context is
large. Do not repeat `domain/application/infra/http` inside every context, and
do not split into multiple crates until Level 2 boundaries are clean enough to
justify compiler-enforced Level 3 crates.

## Accepted Level 2 Shape

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

Adapter rule: `infra/postgres/<context>` must exist for every DB-owning
context. Rewards is the deepest current extraction, not a reason for Postgres
to contain only rewards.

Dependency rule:

```text
http -> application -> domain
infra -> application/domain ports
domain -> no Actix, Diesel, S3, Ethereum, env vars, or HTTP responses
bootstrap -> may know concrete wiring
```

Import guardrails for new or migrated code:

- `domain`: no `actix_web`, Diesel, S3, Ethereum, env vars, or HTTP DTOs.
- `application`: no Actix handlers, Diesel query DSL/schema, provider SDKs, or
  environment reads.
- `http`: no Diesel query builders, legacy repositories, or concrete
  infra/Postgres internals.
- `infra`: no HTTP handlers or HTTP DTO ownership.
- `shared`: no business-context rules.

## Current Completed / Checked State

Live refresh from 2026-06-15:

- No `src/api`, `src/services`, `src/repositories`, or `src/utils` files remain.
- No production `include!` calls remain under `src`.
- No `authenticated_user*` helper usage or direct `pool.get().await` remained
  in `src/http` in the latest scan.
- Remaining integration-test harness cleanup: 3 `tests/*/imports.rs` files and
  28 test `include!` calls.

Completed architecture outcomes:

| Area | Done / checked outcome |
| --- | --- |
| Route ownership | Legacy `src/api` route wrappers are gone; `/api` is assembled from `http/<context>` route configurators. |
| Bootstrap | `main.rs` is thin; state setup, app data, startup checks, and route composition live under `bootstrap`. |
| HTTP boundary | Handlers mostly do extraction, use-case calls, DTO mapping, and local HTTP error mapping. JSON config/errors are centralized. |
| Typed auth | Route handlers use typed auth extractors; `/api/me` preserves its JSON unauthorized response contract. |
| Context rings | Migrated flows now have Level 2 ownership across `domain`, `application`, `infra`, `http`, or `bootstrap` as appropriate. |
| Legacy behavior owners | Legacy `api`, `services`, `repositories`, and `utils` modules were deleted after callers moved to ring owners or direct infra/application replacements. |
| Persistence ownership | Async DB behavior moved out of `models::*`; models are persistence records/change sets, while Diesel logic lives in `infra/postgres/<context>` or test fixtures. |
| Access control | Shared `AccessActor`, `AccessAction`, `AccessScope`, `can(...)`, and `can_any(...)` decisions are used across middleware and migrated use cases. |
| Frontend gates | Platform, organization, learner, teacher-application, and ops UI gates consume backend-derived current-session capabilities. |
| Test harnesses | Production source no longer uses `include!`; many integration harnesses were converted to explicit modules plus `support.rs`. |

## Recent Verified Batches

Detailed history is in git. Keep only compact proof here.

| Batch | Commit | Proof summary |
| --- | --- | --- |
| 287 | `581f1be3` | Removed final request-auth helper; scanned away `authenticated_user*`, stray handler `HttpRequest` parsing, and direct `pool.get().await` in `src/http`. |
| 288-302 | `7cdddb44`..`8b3717e5` | Centralized JSON extractor errors and moved access-control, notifications, operations, identity, KYC, teacher-applications, content, organizations, wallet, reporting, rewards, and learning handlers to typed HTTP results/errors. |
| 303-312 | `19f31541`..`d0d0f32f` | Introduced typed access decisions, migrated reward/wallet/learning/teacher-application/identity/KYC/organization authorization ports, and removed scope-specific permission wrappers. |
| 313-315 | `8ac98ee2`..`4f8eb696` | Repointed frontend action gates to backend current-session capabilities; proved with frontend tests, TypeScript checks, backend session tests, scans, and Cargo gates. |
| 316 | `88308a68` | Converted `course_permissions`, `organization_permissions`, and `middleware_access_control` tests to explicit modules/support. |
| 317 | `6868fe74` | Converted reward/repository reward tests to explicit modules/support and refreshed stale reward API error-envelope assertions. |
| 318 | `7415aebb` | Converted small permission/management harnesses: `model_permission_tests`, `repository_core_tests`, `repository_delegation_tests`, `platform_permissions`, `organization_management`, `course_enrollment_api`. |
| 319 | `dc6b11ea` | Converted `course_content_management` and `student_reward_history`; kept touched files under the 180-line cap. |
| 320 | `ca1c80c9` | Converted `course_join_requests`, `organization_dashboard`, and `worker_upload_jobs`; proved with focused tests, Cargo gates, scans, and line checks. |
| 321 | `61edcae1` | Converted `current_session_api` and `organization_members`; proved with focused tests, Cargo gates, scans, and line checks. |
| 322 | `f3cb4bb1` | Converted `authentication_flow`; split hidden registration test out of shared setup and proved with focused auth tests, Cargo gates, scans, and line checks. |
| 323 | `1dd6ceda` | Converted `teacher_applications` and `organization_teacher_applications`; preserved shared nomination/decision helpers and proved with focused tests, Cargo gates, scans, and line checks. |
| 324 | `5d8f6f19` | Converted `course_discovery` and `blockchain_integration_tests`; split hidden course-list test out of helper setup and proved with focused tests, Cargo gates, scans, and line checks. |
| 325 | `46de0502` | Converted `teacher_course_dashboard` to explicit modules/support; kept dashboard fixtures/helpers crate-local and proved with focused tests, Cargo gates, scans, and line checks. |
| 326 | `4102c76a` | Converted `delegated_permissions` to explicit modules/support; proved with focused delegated-permissions tests, Cargo gates, scans showing no local `include!`/`imports.rs`, and line checks. |
| 327 | `641ddd50` | Converted `reporting_exports` to explicit modules plus `support.rs` and reporting app-data helpers; fixed fixture idempotency keys and proved with focused reporting tests, Cargo gates, scans, and line checks. |
| 328 | this batch | Converted `reward_candidates` to explicit modules/support; fixed reward fixture idempotency keys, refreshed the reward candidate permission-envelope assertion, and proved with focused reward-candidates tests, Cargo gates, scans, and line checks. |

Repeated verification used by completed batches:

- `cargo fmt --all --check`
- focused host tests for touched contexts
- `./scripts/run-host-tests.sh cargo check --lib`
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`
- `git diff --check`
- touched-file size checks for manually maintained non-Markdown files
- source-boundary scans for legacy folders, production `include!`, HTTP
  DB-pool access, request-auth helper usage, and ring import leaks
- frontend tests, TypeScript checks, and permission/capability scans for
  frontend capability batches

## Context Status Snapshot

| Context | Current checked Level 2 status |
| --- | --- |
| Access control | Role/delegation routes, delegated-permission use cases, typed access scopes/actions, shared decisions, middleware wiring, and backend-derived session capabilities are in place. |
| Content | Chapter/content/upload/media/processing handlers use content use cases, Postgres adapters, typed HTTP errors, and one route configurator. |
| Identity | Current session, user profile/list, platform roles, login, verification, registration, reset, JWKS, and auth helper routes have identity HTTP/application/infra ownership. |
| KYC | Status, submission, review, and audit flows have domain rules, application contracts, Postgres adapter/use case, HTTP DTOs, bootstrap wiring, and typed HTTP errors. |
| Learning | Catalog/detail/management/lifecycle/progress/roles/enrollment/assessment/teaching routes use learning contracts, Postgres adapters, HTTP DTOs, bootstrap wiring, and typed HTTP errors. |
| Notifications | Preferences and inbox flows live behind notification application/infra/http ownership; dispatch remains infra-owned. |
| Operations | Health/readiness route composition lives in `http/operations`; persistent readiness/startup concerns are under operations/bootstrap/infra. |
| Organizations | CRUD, courses, members, audit, dashboard, invitations, roles, removal, and teacher-application tracking use organization contracts, Postgres adapters, HTTP DTOs, bootstrap wiring, and typed HTTP errors. |
| Reporting | Platform summary, fraud dashboard, reward dashboard, wallet reconciliation, CSV exports, organization summary, and organization reward dashboard live under reporting application/Postgres/HTTP ownership. |
| Rewards | Policy, fraud-block, history, audit, candidate, decision, compensation, payout, token, wallet-credit, notification, and reconciliation flows have migrated ring owners and access-control-backed checks. |
| Teacher applications | Self-read, audit, list, platform review, submission, decision, organization nomination, and notification fan-out have domain/application/Postgres/HTTP ownership. |
| Wallet | Wallet read/link/audit/token-tax/deposit/retirement/observed-deposit flows have application, Postgres, HTTP/worker ownership, typed HTTP errors, and access-control-backed checks. |

## Active Remaining Work

Keep this list small and evidence-based.

- Finish global integration-test harness cleanup. Remaining files:
  - `tests/reward_execution/imports.rs`
  - `tests/video_upload_flow/imports.rs`
  - `tests/wallet_linking/imports.rs`
- Finish authorization hardening: middleware should remain an early rejection
  optimization, while application use cases remain the real business guard.
- Finish reward boundary hardening: remaining statuses/events/newtypes,
  candidate transitions, DTO leaks, Diesel adapter leaks, and fake-port
  use-case tests need direct evidence.
- Keep auditing data boundaries: Diesel schema/model leaks must stay in infra
  or persistence records, and public API DTOs must remain HTTP-owned.
- Preserve route URLs and response semantics unless a migration note explicitly
  records a behavior change.
- Before each commit/push that advances this objective, refresh this TODO with
  the latest completed proof and active counters.

Suggested next small batch: convert `tests/reward_execution` from
`include!`/`imports.rs` to explicit modules plus `support.rs`, then rerun the
focused reward-execution test and the standard gates.

## Acceptance Criteria

- New routes can be added by touching one context plus shared contracts, not
  `api`, `services`, `repositories`, `models`, `utils`, and frontend
  permission lists at once.
- Domain tests can run without Actix, Diesel, S3, Ethereum, env vars, or a DB
  pool.
- Use-case tests can run with fake ports for authorization, persistence,
  notifications, and time.
- Permission behavior has one backend source of truth.
- New `domain`, `application`, `http`, and `infra` modules pass the import
  checks above.
- Public API DTOs and Diesel records are separate types unless a migration note
  explicitly accepts a temporary leak.
- No new module uses `include!` or `imports.rs`.
- `cargo fmt --all --check` and relevant host tests pass after each context
  migration.

## Non-Goals

- Do not split into microservices yet.
- Do not split into Level 3 crates before Level 2 imports are clean.
- Do not rewrite all contexts at once; keep moving by verified slices.
- Do not add abstractions only to hide Diesel.
- Do not change product behavior during architecture migration unless the
  behavior is covered by a separate TODO item.
