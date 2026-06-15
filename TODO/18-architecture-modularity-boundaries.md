# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Last named verified base: `02388e62`.

Objective: move RustLearn to a Level 2 modular monolith with firm business
boundaries. The target is ownership, not folder volume: HTTP, use cases, domain
rules, adapters, permissions, DTOs, and tests should have clear owners.

## Target Shape

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

Canonical contexts checked so far: `access_control`, `content`, `identity`,
`kyc`, `learning`, `notifications`, `operations`, `organizations`, `reporting`,
`rewards`, `teacher_applications`, `wallet`.

`infra/postgres/<context>` should exist for every DB-owning context. `rewards`
is only the deepest current extraction, not the only Postgres owner.

## Completed / Verified

| Area | Done / checked |
| --- | --- |
| Bootstrap | `main.rs` is thin; startup, state, DB/S3 setup, route composition, and app data registration live under `bootstrap`. |
| Routing | Legacy `src/api` wrappers are gone; `/api` is composed from context route configurators under `http`. |
| HTTP | Migrated handlers use typed extractors, DTOs, typed errors, centralized JSON errors, and use-case calls. Public DTO strings stay HTTP-owned. |
| Application | Migrated workflows use commands, outputs, errors, ports, fake-port tests, and explicit access decisions. |
| Domain | Migrated modules own vocabulary, invariants, status transitions, and validation without framework/provider coupling. |
| Infra | PostgreSQL, object storage, Ethereum, email, worker, and dispatcher code are concrete adapters instead of route owners. |
| Persistence | Async DB behavior moved out of `models::*`; Diesel logic lives in `infra/postgres/<context>` or test fixtures. |
| Access control | Shared `AccessActor`, `AccessAction`, `AccessScope`, `can(...)`, and `can_any(...)` are used by middleware and use cases. |
| Auth | Route handlers use typed auth extractors; `/api/me` keeps its JSON unauthorized response contract. |
| Frontend gates | Platform, organization, learner, teacher-application, and ops gates consume backend-derived current-session capabilities. |
| Tests | Integration harnesses use explicit modules/support helpers instead of `include!` and `imports.rs`. |

Live guardrails checked on 2026-06-15:

- No `src/api`, `src/services`, `src/repositories`, or `src/utils` files remain.
- No production or test `include!` calls remain under `src` or `tests`.
- No `tests/*/imports.rs` files remain.
- No `authenticated_user*` helper usage remains in `src/http`.
- No direct `pool.get().await` remains in `src/http`.
- Boundary scans check ring import leaks from `domain` and `application`.

## Verified Ledger

Detailed history belongs in git; this ledger keeps the architectural proof.

| Batches | Commits | Proof |
| --- | --- | --- |
| 287-331 | `581f1be3`..`57215a3c` | Removed final request-auth helper; centralized JSON extractor errors; moved major context handlers to typed HTTP results/errors; introduced typed access decisions; repointed frontend gates to backend current-session capabilities; normalized integration harnesses away from `include!` and `imports.rs`. Proof included focused frontend/backend tests, Cargo gates, line checks, and boundary scans. |
| 332-336 | `5a1643fa`..`2287a1f9` | Typed reward payout candidate, teacher decision, amount decision, submission, course/platform candidate, and student reward-history statuses at the infra/application boundary. HTTP kept public string DTO contracts. Proof included focused use-case/mapper/integration tests, Cargo gates, no-run integration compile, line checks, and boundary scans. |
| 337 | `02388e62` | Typed reward candidate audit `from_status`/`to_status` and reconciliation `final_status` at the infra/application boundary. HTTP kept public audit-status strings and DB audit inserts stayed infra-owned. Proof: audit mapper known/unknown-status tests, candidate-audit and reconciliation use-case tests, `reward_candidate_audit`, `reward_candidates`, `reward_execution` reconciliation integration, `api_routing`, Cargo format/check gates, no-run integration compile, line checks, and boundary scans. |
| 338 | same commit as this TODO update | Typed reward candidate source/event, audit event, reward-history event, payout event, payment strategy, and payout method across application outputs/ports. Infra parses persisted vocabulary before crossing into application; HTTP keeps public string DTOs. Proof: reward vocabulary helper tests, candidate/history/audit/payout mapper tests, source-scope use-case tests, `reward_candidates`, `reward_candidate_audit`, `reward_course_candidates`, `student_reward_history`, `reward_execution`, `api_routing`, Cargo format/check gates, no-run integration compile, line checks, and boundary scans. |

Standard proof set for each verified batch:

- `cargo fmt --all --check`
- focused host tests for touched contexts
- `./scripts/run-host-tests.sh cargo check --lib`
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`
- `git diff --check`
- touched-file line checks for manually maintained non-Markdown files
- boundary scans for legacy folders, `include!`, HTTP DB-pool access,
  request-auth helper usage, and ring import leaks

## Active Remaining Work

- Rewards: convert remaining policy, fraud-block, token-confirmation, credit,
  and notification event/payment strings into domain vocabulary where they
  represent business state rather than public query strings.
- Rewards: inspect transition records and reporting-facing reward rows for
  business-state strings crossing into application.
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
