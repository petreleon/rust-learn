# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Last named verified base: `dbc5259f`.

Objective: move RustLearn to a Level 2 modular monolith with firm business
boundaries. The target is ownership, not folder volume: HTTP, use cases, domain
rules, adapters, permissions, DTOs, and tests should have clear owners.

## Current Rules

Top-level rings stay stable; bounded contexts live inside those rings as
granular modules where needed.

```text
src/
  bootstrap/    process wiring, app state, startup, route composition
  shared/       tiny cross-context primitives only
  domain/       pure vocabulary, invariants, transitions
  application/  use cases, commands, outputs, errors, ports
  infra/        Postgres, object storage, Ethereum, email, dispatchers
  http/         Actix routes, extractors, DTOs, response mapping
```

Dependency rule:

```text
http -> application -> domain
infra -> application/domain ports
bootstrap -> concrete wiring
domain -> no Actix, Diesel, S3, Ethereum, env vars, or HTTP responses
```

Canonical contexts checked so far:

```text
access_control, content, identity, kyc, learning, notifications,
operations, organizations, reporting, rewards, teacher_applications, wallet
```

`infra/postgres/<context>` should exist for every DB-owning context. `rewards`
is only the deepest current extraction, not the only Postgres owner.

## Already Done

| Area | Status |
| --- | --- |
| Bootstrap | `main.rs` is thin; startup, state, DB/S3 setup, route composition, and app data registration live under `bootstrap`. |
| Routing | Legacy `src/api` wrappers are gone; `/api` is composed from context route configurators under `http`. |
| HTTP | Migrated handlers use typed extractors, DTOs, typed errors, centralized JSON errors, and use-case calls. |
| Application | Migrated workflows use commands, outputs, errors, ports, fake-port tests, and explicit access decisions. |
| Domain | Migrated modules own vocabulary, invariants, status transitions, and validation without framework/provider coupling. |
| Infra | PostgreSQL, object storage, Ethereum, email, worker, and dispatcher code are concrete adapters instead of route owners. |
| Persistence | Async DB behavior moved out of `models::*`; Diesel logic lives in `infra/postgres/<context>` or test fixtures. |
| Access control | Shared `AccessActor`, `AccessAction`, `AccessScope`, `can(...)`, and `can_any(...)` are used by middleware and use cases. |
| Auth | Route handlers use typed auth extractors; `/api/me` keeps its JSON unauthorized response contract. |
| Frontend gates | Platform, organization, learner, teacher-application, and ops gates consume backend-derived current-session capabilities. |
| Tests | Integration harnesses use explicit modules/support helpers instead of `include!` and `imports.rs`. |

## Already Checked

Live guardrails checked on 2026-06-15:

- No `src/api`, `src/services`, `src/repositories`, or `src/utils` files remain.
- No production or test `include!` calls remain under `src` or `tests`.
- No `tests/*/imports.rs` files remain.
- No `authenticated_user*` helper usage remains in `src/http`.
- No direct `pool.get().await` remains in `src/http`.
- Boundary scans check ring import leaks from `domain` and `application`.

Standard proof set used for verified batches:

- `cargo fmt --all --check`
- focused host tests for touched contexts
- `./scripts/run-host-tests.sh cargo check --lib`
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`
- `git diff --check`
- touched-file line checks for manually maintained non-Markdown files
- boundary scans for legacy folders, `include!`, HTTP DB-pool access,
  request-auth helper usage, and ring import leaks

## Verified Ledger

Detailed history belongs in git; this table keeps the architectural proof.

| Batch | Commit | Proof |
| --- | --- | --- |
| 287 | `581f1be3` | Removed final request-auth helper and scanned away `authenticated_user*`, stray handler `HttpRequest` parsing, and direct `pool.get().await` in `src/http`. |
| 288-302 | `7cdddb44`..`8b3717e5` | Centralized JSON extractor errors and moved major context handlers to typed HTTP results/errors. |
| 303-312 | `19f31541`..`d0d0f32f` | Introduced typed access decisions and removed scope-specific permission wrappers from migrated use cases. |
| 313-315 | `8ac98ee2`..`4f8eb696` | Repointed frontend action gates to backend current-session capabilities and proved with frontend/backend checks. |
| 316-331 | `88308a68`..`57215a3c` | Normalized integration harnesses away from `include!`/`imports.rs` across delegated permissions, reporting, rewards, wallet linking, video upload, and related flows. |
| 332 | `5a1643fa` | Typed reward payout candidate status at the infra/application boundary. |
| 333 | `247a5096` | Typed teacher reward decision output status; infra parses persisted status and HTTP maps it back to the public string. |
| 334 | `a3a7bbbf` | Typed reward amount decision output status; infra parses persisted status and HTTP preserves the public string contract. |
| 335 | `dbc5259f` | Typed reward candidate submission output status; creation and idempotent replay parse persisted status before use-case output. |
| 336 | same commit as this TODO update | Typed course candidate, platform candidate, and student reward-history read-model status at the infra/application boundary; HTTP keeps public string DTOs. Proof: mapper known/unknown-status tests, course/platform/history use-case filters, `reward_course_candidates`, platform candidate integration, `student_reward_history`, `api_routing`, Cargo format/check gates, no-run integration compile, line checks, and boundary scans. |

## Active Remaining Work

- Rewards: type remaining status/string boundaries such as candidate audit
  `to_status`, reconciliation `final_status`, and any transition output that
  still crosses application as a raw persisted status string.
- Rewards: keep converting event/source/method strings into domain vocabulary
  where they represent business state rather than public query strings.
- Persistence: keep Diesel schema/model leakage inside infra or persistence
  records.
- HTTP: keep public API DTOs HTTP-owned and separate from Diesel records.
- Authorization: keep application use cases as the real guard; middleware
  remains early rejection only.
- Behavior: preserve route URLs and response semantics unless a migration note
  records a behavior change.

## Constraints

- Do not split into microservices or Level 3 crates yet.
- Do not rewrite all contexts at once.
- Do not add abstractions only to hide Diesel.
- Do not change product behavior during architecture migration unless another
  TODO item covers it.
