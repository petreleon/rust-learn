# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base before current batch: `a489fcb7`
(`Move Postgres schema and pool under infra`).

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
  to `src/infra/postgres`; root `db` exports removed; `diesel.toml`, bootstrap,
  worker, infra adapters, and DB-backed tests use the Postgres infra path.

Proof for the latest pushed architecture batch:

- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- Focused tests: `--lib connection::tests`, `--test db_version_control`,
  `--test repository_core_tests`, `--test worker_upload_jobs`,
  `--test health_readiness`, and `--test api_routing`.
- Scans: no old `crate::db` / `rust_learn::db` / `src/db` source references, no
  root `src/db`, no domain/application infra/HTTP/DB leaks, no HTTP DB leaks,
  no `include!`, no `imports.rs`, no application DTO/Actix leaks, and no
  maintained Rust file over 180 lines.

## Current Verified Batch

Included problems:

- `make migrate` and `make migrate-redo` still used host Diesel, even though
  schema generation now belongs to the Postgres infra path and development
  workflows should be Docker-first.

Fixes:

- Added a `diesel_cli` Docker build target and a small
  `docker-compose.tools.yml` Diesel tool service.
- Updated `make migrate` and `make migrate-redo` to start Compose PostgreSQL
  and run Diesel through the Compose tool container.
- Updated README and contributor guidance to make `make migrate` the primary
  development command.

Proof:

- `make -n migrate` and `make -n migrate-redo` expand to Docker Compose Diesel
  commands.
- `docker compose -f docker-compose.yml -f docker-compose.tools.yml config
  --services`.
- `docker compose -f docker-compose.yml -f docker-compose.tools.yml build
  diesel`.
- `docker compose -f docker-compose.yml -f docker-compose.tools.yml run --rm
  --no-deps diesel diesel --version`.
- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`; manual non-Markdown files remain <=180 lines.

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
- Rough remaining effort: 2-4 focused batches plus the final requirement audit.
