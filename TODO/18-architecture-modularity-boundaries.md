# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Last verified commit: `5a1643fa`.

Objective: move RustLearn from a technically layered monolith to a Level 2
modular monolith with firm business boundaries. The goal is ownership, not
folder volume: HTTP, use cases, domain rules, adapters, permissions, DTOs, and
tests should have clear owners.

## Target Shape

Top-level architectural rings stay stable; bounded contexts live inside those
rings as needed. Do not repeat `domain/application/infra/http` as top-level
folders inside every context, and do not split crates until Level 2 imports are
clean enough for Level 3.

```text
src/
  bootstrap/    process wiring, app state, startup, route composition
  shared/       tiny cross-context primitives only
  domain/       pure business vocabulary, invariants, transitions
  application/  use cases, commands, outputs, errors, ports
  infra/        Postgres, object storage, Ethereum, email, dispatchers
  http/         Actix routes, extractors, DTOs, response mapping
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
bootstrap -> concrete wiring
domain -> no Actix, Diesel, S3, Ethereum, env vars, or HTTP responses
```

`infra/postgres/<context>` should exist for every DB-owning context. `rewards`
is only the deepest current extraction; it is not the only Postgres owner.

## Already Done

| Area | Compact status |
| --- | --- |
| Bootstrap | `main.rs` is thin; state setup, route composition, startup checks, DB/S3 wiring, and app data registration live under `bootstrap`. |
| Routing | Legacy `src/api` wrappers are gone; `/api` is composed from context route configurators under `http`. |
| HTTP | Migrated handlers extract typed inputs, call use cases, map DTOs, and return typed HTTP errors. JSON config/errors are centralized. |
| Application | Migrated workflows use commands, outputs, errors, ports, fake-port tests, and explicit access decisions. |
| Domain | Migrated domain code owns vocabulary, invariants, status transitions, and business validation without framework/provider coupling. |
| Infra | PostgreSQL, object storage, Ethereum, email, workers, and dispatchers are concrete adapters rather than route owners. |
| Persistence | Async DB behavior moved out of `models::*`; Diesel logic lives in `infra/postgres/<context>` or test fixtures. |
| Access control | Shared `AccessActor`, `AccessAction`, `AccessScope`, `can(...)`, and `can_any(...)` are used by middleware and migrated use cases. |
| Typed auth | Route handlers use typed auth extractors; `/api/me` preserves its JSON unauthorized response contract. |
| Frontend gates | Platform, organization, learner, teacher-application, and ops gates consume backend-derived current-session capabilities. |
| Tests | Integration harnesses use explicit modules/support helpers instead of `include!` and `imports.rs`. |

Checked Level 2 contexts:

```text
access_control, content, identity, kyc, learning, notifications,
operations, organizations, reporting, rewards, teacher_applications, wallet
```

## Already Checked

Live checks recorded on 2026-06-15:

- No `src/api`, `src/services`, `src/repositories`, or `src/utils` files remain.
- No production `include!` calls remain under `src`.
- No `authenticated_user*` helper usage remains in `src/http`.
- No direct `pool.get().await` remains in `src/http`.
- No `tests/*/imports.rs` files remain.
- No test `include!` calls remain.

Recent proof set:

- `cargo fmt --all --check`
- focused host tests for touched contexts
- `./scripts/run-host-tests.sh cargo check --lib`
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`
- `git diff --check`
- touched-file line checks for manually maintained non-Markdown files
- boundary scans for legacy folders, `include!`, HTTP DB-pool access,
  request-auth helper usage, and ring import leaks

## Verified Batches

Detailed history belongs in git; this table keeps only the architectural proof.

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
| 329 | `fc018ef7` | Converted `reward_execution`; focused tests, Cargo gates, scans, and line checks passed. |
| 330 | `6b0092a2` | Converted `wallet_linking`; refreshed stale KYC conflict assertion and proved with focused wallet tests, Cargo gates, scans, and line checks. |
| 331 | `57215a3c` | Converted `video_upload_flow`; moved content app wiring and sample-video helpers behind explicit modules and proved with focused tests, gates, scans, and line checks. |
| 332 | `5a1643fa` | Typed reward payout candidate status at the infra/application boundary and proved with fake-port tests, mapper tests, Cargo gates, scans, and line checks. |
| 333 | this batch | Typed teacher reward decision output status at the infra/application boundary; infra parses persisted candidate status before returning use-case output, HTTP maps it back to the public string, and proof covered teacher-decision filters, mapper status tests, reward/delegation integration crates, Cargo gates, scans, and line checks. |

## Remaining Work

- Keep application use cases as the real authorization guard; middleware should
  remain an early rejection optimization.
- Continue rewards hardening around remaining events/newtypes, candidate
  transitions, DTO leaks, Diesel adapter leaks, and fake-port use-case tests.
- Keep Diesel schema/model leakage inside infra or persistence records.
- Keep public API DTOs HTTP-owned and separate from Diesel records.
- Preserve route URLs and response semantics unless a migration note explicitly
  records a behavior change.

## Acceptance Criteria

- New routes can be added by touching one context plus shared contracts.
- Domain tests run without Actix, Diesel, S3, Ethereum, env vars, or a DB pool.
- Use-case tests run with fake ports for auth, persistence, notifications, and
  time.
- Permission behavior has one backend source of truth.
- Ring modules pass the import guardrails above.
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
