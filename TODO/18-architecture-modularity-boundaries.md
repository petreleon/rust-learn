# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base before current batch: `f02bfd1b`
(`Move Actix middleware under HTTP ring`).

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
- `004d1037`: KYC audit event vocabulary and organization member audit event
  read/write/output vocabulary.
- `7693fb71`: readiness dependency labels are abstract; wallet/platform reward
  application command/output names are separated from HTTP DTO names.
- `87c4ca4b`: reward evidence and reward audit metadata moved into domain
  reward vocabulary; unused top-level `shared` module removed.
- `18c1796c`: all Diesel model records moved from top-level `src/models` to
  `src/infra/postgres/models`; root `models` exports removed.
- `a489fcb7`: Diesel schema and pool construction moved from top-level `src/db`
  to `src/infra/postgres`; root `db` exports removed.
- `216b9529`: `make migrate` and `make migrate-redo` now run Diesel through a
  Compose tool container; Dockerfile/Compose/docs updated and verified.
- `2574c1c6`: DB setup/version updates moved from `src/config/db_setup` to
  `src/infra/postgres/operations/db_setup`; startup and docs now import DB
  versioning through Postgres infra.
- `0dcd66e5`: Make targets are the primary development interface for Diesel
  workflow; `make diesel-compose`, `make migration-generate`, `make
  fmt-compose`, and `make mock-email` added; README/agent guidance no longer
  requires host Diesel for schema generation.
- `f02bfd1b`: Actix middleware moved from top-level `src/middlewares` to
  `src/http/middlewares`; root exports removed; routes/tests now import through
  `http::middlewares`.

Proof for recent pushed batches:

- Formatting, lib check, app binary check, and `cargo test --tests --no-run`.
- Focused DB/Postgres tests: connection unit tests, `db_version_control`,
  repository core, worker upload jobs, readiness, and API routing.
- Docker tooling checks: Make dry-runs, Compose config, Diesel tool image build,
  and Compose Diesel `--version`.
- Scans: no old `crate::db` / `rust_learn::db` / `src/db` source references, no
  root `src/db`, no domain/application infra/HTTP/DB leaks, no HTTP DB leaks,
  no `include!`, no `imports.rs`, no application DTO/Actix leaks, and no
  maintained Rust file over 180 lines.

## Current Verified Batch

Included problems:

- `src/config` was still a top-level non-ring module.
- The only remaining config-owned data was pure role/permission vocabulary,
  which belongs in domain access-control vocabulary.
- `src/lib.rs` and `src/main.rs` exposed `config` as a root API.
- HTTP routes, Postgres adapters, setup updates, and tests imported role and
  permission names through `config::constants`.
- `PERMISSIONS.md` missed `REVIEW_KYC_SUBMISSIONS` even though the enum and
  migrations include it.

Fixes:

- Moved `Permissions` and `Roles` to
  `src/domain/access_control/{permissions,roles}.rs`.
- Exported the moved vocabulary through `src/domain/access_control/mod.rs`.
- Removed `src/config` and root `config` exports.
- Updated HTTP, infra, setup, and test imports to use
  `domain::access_control::{permissions,roles}`.
- Updated README/AGENTS repository maps.
- Added `REVIEW_KYC_SUBMISSIONS` to `PERMISSIONS.md` for platform ADMIN and
  SUPER_ADMIN to match the seeded migration.

Deferred problems:

- Raw permission literals still exist across application use cases and current
  session capability definitions. Consolidating those behind typed domain
  vocabulary spans multiple product flows and should be a separate vocabulary
  batch or final-audit item, not mixed into this mechanical module move.
- The newer `domain::access_control::Permission` enum still overlaps with the
  moved `Permissions` catalog. Unifying those types would be a broader API
  cleanup because existing authorization use cases intentionally use the smaller
  typed subset.

Proof:

- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo test --test
  permission_catalog_consistency`.
- `./scripts/run-host-tests.sh cargo test --test platform_permissions_unit`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features
  app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`.
- Scans: no `src/config` directory, no `crate::config` /
  `rust_learn::config` / `config::constants` references in source/tests/docs,
  root `pub mod config` removed from `src/lib.rs` and `src/main.rs`, no
  domain/application Actix/DB/HTTP leaks, and no maintained Rust file over 180
  lines.

## Remaining Work

- Audit TODO/18 requirement-by-requirement against current code and pushed
  evidence before calling Level 2 complete.
- Continue raw vocabulary scans for migrated contexts; batch only one
  architectural concern at a time.
- Keep public API DTOs HTTP-owned and separate from application commands and
  outputs.
- Keep use cases as the real authorization guard; middleware remains early
  rejection.
- Do not jump to Level 3 crates/microservices or abstractions that only move
  files around.
- Rough remaining effort: 0-1 focused vocabulary batch plus the final
  requirement audit.
