# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base before current batch: `ae518ea0`
(`Move permission catalog into domain`).

Objective: finish RustLearn as a Level 2 modular monolith. Keep the main rings
`domain`, `application`, `infra`, `http`, and `bootstrap`; use granular
submodules only where a context or use case has real ownership.

## Boundary Rules

- Flow: `http -> application -> domain`; `infra` implements application ports;
  `bootstrap` wires concrete state.
- `domain`: pure vocabulary, invariants, transition helpers.
- `application`: commands, outputs, errors, ports, authorization decisions,
  orchestration.
- `infra`: Postgres, object storage, Ethereum, email, workers, dispatchers.
- `http`: Actix routes, extractors, DTOs, request/response mapping.
- Forbidden in `domain`/`application`: Actix, Diesel, S3, Ethereum, env vars,
  HTTP responses, request extractors, concrete pools, and web state.
- Keep manual non-Markdown files <=180 lines.
- Batch only safe, reviewable architectural concerns.

## Completed / Verified

- `581f1be3`..`57215a3c`: auth helpers, extractor errors, typed HTTP
  results/errors, access decisions, frontend gates, and test harness cleanup.
- `5a1643fa`..`b1413b5d`: reward payout/read/audit/reconciliation/source/event,
  payment, policy, fraud, and token-confirmation vocabulary.
- `53a3fd67`..`d1ca7530`: platform reward dashboards/exports, teacher and
  delegated-permission CSV vocabulary, fraud dashboard scope, wallet owner
  outputs.
- `0db0000c`..`28019ba3`: delegated-permission scope, wallet statuses/events,
  token event boundaries, reward command vocabulary, teacher-application event
  vocabulary, and external token event outputs.
- `004d1037`..`87c4ca4b`: KYC/member audit vocabulary, readiness labels,
  application command/output naming, reward evidence vocabulary, and unused
  `shared` module removal.
- `18c1796c`..`2574c1c6`: Diesel models, schema/pool, and DB setup/versioning
  moved under `src/infra/postgres`; root `models`, `db`, and config DB exports
  removed.
- `216b9529` and `0dcd66e5`: Diesel development workflow now runs through Make
  and Compose (`make migrate`, `make migrate-redo`, `make diesel-compose`,
  `make migration-generate`) without requiring host Diesel.
- `f02bfd1b`: Actix middleware moved from top-level `src/middlewares` to
  `src/http/middlewares`; root exports removed; routes/tests now import through
  `http::middlewares`.
- `ae518ea0`: role/permission catalog moved from top-level `src/config` to
  `src/domain/access_control`; root `config` exports removed; HTTP, infra, and
  tests import through domain vocabulary; `PERMISSIONS.md` includes
  `REVIEW_KYC_SUBMISSIONS`.

Proof for completed pushed work:

- Formatting, lib check, app binary check, and `cargo test --tests --no-run`.
- Focused tests for DB/Postgres, routing, permissions, middleware, readiness,
  worker upload jobs, and affected product contexts.
- Docker tooling checks: Make dry-runs, Compose config, Diesel tool image build,
  and Compose Diesel `--version`.
- Scans: no old `crate::db` / `rust_learn::db` / `src/db` source references, no
  root `src/db`, `src/models`, `src/config`, or `src/middlewares`, no
  domain/application infra/HTTP/DB leaks, no HTTP DB leaks, no `include!`, no
  `imports.rs`, no application DTO/Actix leaks, and no maintained Rust file over
  180 lines.

## Current Verified Batch

Included problems:

- Current-session backend capability definitions duplicated permission catalog
  strings in `src/application/identity/current_session/capabilities`.
- Current-session access summary hardcoded `SUBMIT_TEACHER_APPLICATION`.
- KYC review and audit handlers hardcoded `REVIEW_KYC_SUBMISSIONS`.
- The domain role/permission catalog was value-like but not `Copy`/`Eq`, making
  catalog-backed static metadata awkward.

Fixes:

- `CapabilityDefinition.permissions` now stores
  `domain::access_control::permissions::Permissions` variants.
- Current-session capability output still exposes `Vec<String>`, preserving the
  API contract while deriving names from the domain catalog.
- KYC review queue, decision, and audit authorization use
  `Permissions::REVIEW_KYC_SUBMISSIONS`.
- `Permissions` and `Roles` derive `Clone`, `Copy`, and `Eq` for safe static
  catalog use.

Deferred problems:

- Domain delegation permission rules still normalize a scoped raw string policy
  list. That is a separate domain policy surface and should be audited as its
  own batch if more vocabulary work is needed.
- Some tests intentionally assert serialized permission strings. Those remain as
  API-contract checks rather than production vocabulary duplication.
- The smaller `domain::access_control::Permission` enum still overlaps with the
  full `Permissions` catalog. Unifying them would be a broader authorization API
  cleanup, not a safe side effect of this batch.

Proof:

- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo test --test current_session_api`.
- `./scripts/run-host-tests.sh cargo test --test kyc_review`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features
  app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`.
- Scans: no raw permission string literals remain in current-session/KYC
  application code; no maintained Rust file over 180 lines; domain/application
  boundary scan found no concrete dependency leaks, only the domain vocabulary
  variant `MANAGE_S3_OBJECTS`.

## Remaining Work

- Audit TODO/18 requirement-by-requirement against current code and pushed
  evidence before calling Level 2 complete.
- Re-scan raw vocabulary across remaining migrated contexts; only batch more
  cleanup if it is cohesive, behavior-preserving, and reviewable.
- Decide whether the overlapping `Permission` and `Permissions` domain enums
  should stay separate or be unified through a deliberate authorization API
  cleanup.
- Keep public API DTOs HTTP-owned and separate from application commands and
  outputs.
- Keep use cases as the real authorization guard; middleware remains early
  rejection.
- Do not jump to Level 3 crates/microservices or abstractions that only move
  files around.
- Rough remaining effort: final requirement audit, plus at most one focused
  vocabulary/API cleanup if the audit proves it necessary.
