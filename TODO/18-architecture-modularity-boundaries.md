# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base before current batch: `18c1796c`
(`Move Diesel models under postgres infra`).

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
  `src/infra/postgres/models`; root `models` exports removed; infra, worker,
  Diesel associations, and DB-backed tests use `infra::postgres::models`.

## Current Verified Batch

Included problems:

- Top-level `src/db` owned Diesel schema and pool construction outside the
  Level 2 rings.
- Runtime wiring, worker code, infra adapters, and DB-backed tests imported the
  root `db` namespace.
- Diesel CLI and README still pointed schema maintenance at `src/db/schema.rs`.

Fixes:

- Moved pool construction to `src/infra/postgres/connection.rs`.
- Moved generated Diesel schema to `src/infra/postgres/schema.rs`.
- Re-exported `DbPool`, `database_url_from_env`, `establish_connection`, and
  `try_establish_connection` from `infra::postgres`.
- Removed root `db` modules from `src/lib.rs` and `src/main.rs`.
- Updated bootstrap, worker, infra adapters, DB-backed tests, `diesel.toml`, and
  `README.md` to the Postgres infra path.

Deferred problems:

- DB-backed test fixtures still use Diesel directly; that is a test-fixture
  ergonomics concern, not the production Postgres ownership boundary moved here.
- Remaining application `Request` wording is deferred to a product-language
  audit because the current batch is only DB/Postgres ownership.

Proof:

- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- Focused tests: `--lib connection::tests`, `--test db_version_control`,
  `--test repository_core_tests`, `--test worker_upload_jobs`,
  `--test health_readiness`, and `--test api_routing`.
- `git diff --check`.
- Scans: no old `crate::db` / `rust_learn::db` / `src/db` source references, no
  root `src/db`, Postgres schema and connection files present, no
  domain/application infra/HTTP/DB leaks, no HTTP DB leaks, no `include!`, no
  `imports.rs`, no application DTO/Actix leaks, and no maintained Rust file over
  180 lines.

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
