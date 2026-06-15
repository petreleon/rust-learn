# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base before current batch: `2574c1c6`
(`Move DB setup updates under Postgres infra`).

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

- Development docs still mixed Make targets with raw Docker Compose/host Cargo
  examples as primary commands.
- Migration guidance implied host Diesel was still an option, even though
  schema generation should be driven by Compose Diesel.
- The Makefile had `migrate`/`migrate-redo`, but no discoverable Make target for
  arbitrary Diesel CLI work or migration generation.
- Agent guidance still referenced old DB paths such as `src/db/schema.rs` and
  `db::establish_connection()`.

Fixes:

- Added `make diesel-compose DIESEL_ARGS='...'` as the Make-owned Diesel CLI
  passthrough through the Compose tool container.
- Added `make migration-generate NAME=...`, `make fmt-compose`, and
  `make mock-email` helper targets.
- Rewired `make migrate` and `make migrate-redo` to delegate to
  `make diesel-compose`.
- Fixed `make help` so targets from included `mk/*.mk` files display by target
  name instead of include filename.
- Updated README, AGENTS, and Copilot guidance so Make targets are primary,
  host Diesel is not required, and schema generation writes
  `src/infra/postgres/schema.rs`.

Deferred problems:

- `make fmt`, `make dev-run`, and `make dev-worker` still use host toolchains by
  design, but they are Make-owned entrypoints.
- Runtime container migrations still use the Diesel binary inside the runtime
  image with an empty runtime config; this is a deployment concern, not the
  development schema-generation path.

Proof:

- Make dry-runs: `migrate`, `migrate-redo`, `diesel-compose
  DIESEL_ARGS='print-schema'`, `migration-generate NAME=create_learning_paths`,
  `fmt-compose`, and `mock-email`.
- `make help` shows the new included targets by name.
- `docker compose -f docker-compose.yml -f docker-compose.tools.yml config
  --services` includes the `diesel` tool service.
- `git diff --check`.
- Scans over README, AGENTS, Copilot guidance, Makefile, and `mk/`: no docs
  still requiring host Diesel, no old `src/db/schema.rs`, no
  `db::establish_connection()` guidance, and no raw Compose test/log commands
  presented as the primary workflow.

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
