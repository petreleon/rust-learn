# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base before current batch: `216b9529`
(`Run Diesel migrations through Compose`).

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

- `src/config/db_setup` owned DB version updates while depending on
  `diesel_async::AsyncPgConnection` and Postgres adapters.
- Startup imported concrete DB setup through `config`, which blurred the
  config/infra boundary.
- Contributor and testing notes pointed at the old config path.

Fixes:

- Moved DB setup/version updates to
  `src/infra/postgres/operations/db_setup`.
- Removed the `db_setup` export from `src/config`.
- Updated startup, Postgres operations exports, contributor notes, and testing
  TODO references to the new infra path.

Deferred problems:

- `config/constants` still owns role/permission strings; it has no concrete DB
  dependency and needs a broader policy/constants placement audit before moving.
- Bootstrap still receives concrete startup connections; that belongs to a
  separate startup/deployment audit because bootstrap is allowed to wire
  concrete dependencies.

Proof:

- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- Focused tests: `--lib operations::db_setup` and `--test db_version_control`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`.
- Scans: no old `config/db_setup` path, no `config::db_setup`, no concrete
  Postgres/Diesel imports in `config`, `application`, or `domain`, and no
  maintained Rust file over 180 lines.

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
- Rough remaining effort: 1-3 focused batches plus the final requirement audit.
