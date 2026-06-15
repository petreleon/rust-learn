# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Last named verified base: `986b1686`.

Objective: move RustLearn to a Level 2 modular monolith with firm business
boundaries. Ownership matters more than folder volume.

## Shape

```text
src/
  bootstrap/    process wiring, app state, startup, route composition
  shared/       tiny cross-context primitives only
  domain/       pure vocabulary, invariants, transitions
  application/  use cases, commands, outputs, errors, ports
  infra/        Postgres, object storage, Ethereum, email, dispatchers
  http/         Actix routes, extractors, DTOs, response mapping
```

```text
http -> application -> domain
infra -> application/domain ports
bootstrap -> concrete wiring
domain -> no Actix, Diesel, S3, Ethereum, env vars, or HTTP responses
```

Checked contexts: `access_control`, `content`, `identity`, `kyc`, `learning`,
`notifications`, `operations`, `organizations`, `reporting`, `rewards`,
`teacher_applications`, `wallet`.

## Done / Checked

| Area | Compact status |
| --- | --- |
| Bootstrap/routing | `main.rs` is thin; setup and route composition live under `bootstrap`; legacy `src/api` wrappers are gone. |
| HTTP | Migrated handlers use typed extractors, DTOs, typed errors, centralized JSON errors, and application use cases. |
| Application/domain | Migrated workflows use commands, outputs, ports, fake-port tests, access decisions, pure vocabulary, invariants, and transitions. |
| Infra/persistence | Concrete adapters own PostgreSQL, object storage, Ethereum, email, worker, and dispatcher code; Diesel access moved out of `models::*`. |
| Auth/access | Shared access-control vocabulary is used by middleware and use cases; route handlers use typed auth extractors; `/api/me` keeps its unauthorized JSON contract. |
| Frontend/tests | Frontend gates consume backend current-session capabilities; test harnesses no longer use `include!` or `imports.rs`. |

- No `src/api`, `src/services`, `src/repositories`, or `src/utils` files remain.
- No production or test `include!` calls remain under `src` or `tests`.
- No `tests/*/imports.rs` files remain.
- No `authenticated_user*` helper usage remains in `src/http`.
- No direct `pool.get().await` remains in `src/http`.
- Boundary scans check ring import leaks from `domain` and `application`.

## Verified Ledger

| Batches | Commits | Proof |
| --- | --- | --- |
| 287-331 | `581f1be3`..`57215a3c` | Removed final request-auth helper, centralized JSON extractor errors, moved major context handlers to typed HTTP results/errors, introduced typed access decisions, repointed frontend gates, and normalized integration harnesses away from `include!`/`imports.rs`. |
| 332-336 | `5a1643fa`..`2287a1f9` | Typed reward payout candidate, teacher decision, amount decision, submission, course/platform candidate, and student reward-history statuses at the infra/application boundary. |
| 337 | `02388e62` | Typed reward candidate audit `from_status`/`to_status` and reconciliation `final_status` at the infra/application boundary. |
| 338 | `986b1686` | Typed reward candidate source/event, audit event, reward-history event, payout event, payment strategy, and payout method across application outputs/ports. Infra parses persisted vocabulary before crossing into application; HTTP keeps public string DTOs. |
| 339 | same commit as this TODO update | Typed reward policy scope/event/payment strategy, fraud-block scope/audit event, and token-confirmation transaction type across application outputs/ports. Infra validates persisted vocabulary before crossing into application; HTTP keeps public string DTOs. |

Standard proof set: `cargo fmt --all --check`, focused host tests, Cargo
library/binary checks, integration no-run compile, `git diff --check`, touched
file line checks, and boundary scans.

## Active Remaining Work

- Rewards: convert remaining credit, notification event/payment, transition,
  and reporting-facing business strings into domain vocabulary where they cross
  infra/application boundaries.
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
