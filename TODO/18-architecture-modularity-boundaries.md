# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base before current batch: `76cbc9f1`
(`Unify reward authorization permissions catalog`).

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
- `58749d50`: current-session capabilities and KYC review/audit authorization
  now use `Permissions` catalog values while preserving string API output.
- `380f9424`: production application permission checks and denied-permission
  payloads now use `Permissions` catalog values across delegated permissions,
  identity, learning, organization, teacher-application, and reward use cases.
- `4e8fd741`: HTTP JWT middleware now verifies bearer tokens through an
  application `AuthTokenVerifier` port wired by bootstrap, not by importing
  infra directly. Diesel development commands also use Make/Compose as the
  primary path, with `make schema` regenerating and formatting
  `src/infra/postgres/schema.rs` inside the tool container.
- `76cbc9f1`: reward/wallet authorization and fraud-block notification
  recipient policy now use the canonical `Permissions` catalog; the duplicate
  `domain::access_control::permission::Permission` subset enum was removed.

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

- `domain::access_control::delegation::permission_rules` still hard-coded the
  delegatable reward permission set as raw strings.
- Scope validation for delegated platform, organization, and course permissions
  repeated those string literals in `matches!` blocks.
- This could drift from the canonical `Permissions` catalog after the duplicate
  subset enum was removed.

Fixes:

- Delegation permission normalization now parses input into `Permissions`.
- Delegatable reward permissions and scope-specific delegation rules now use
  `Permissions` variants directly.
- Public/persisted boundaries still return and filter permission names as
  strings, preserving the API and DB contract.
- String literals remain only in tests that assert public permission-name
  behavior.

Deferred problems:

- Opaque `serde_json::Value` evidence/metadata fields remain in domain and
  application outputs. They are not Actix DTOs, but the final audit should
  confirm they are intentional product payloads rather than HTTP leakage.

Proof:

- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo test --lib delegation`.
- `./scripts/run-host-tests.sh cargo test --lib manage_delegated_permissions`.
- `./scripts/run-host-tests.sh cargo test --test delegated_permissions --test
  reward_management_api`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features
  app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`.
- Scans: no hard-coded delegated reward permission literals remain in
  production delegation policy; no `crate::infra` / `rust_learn::infra` imports
  in `src/http`; domain/application boundary scan found no concrete dependency
  leaks, only the domain vocabulary variant `MANAGE_S3_OBJECTS`; no maintained
  Rust file over 180 lines.

## Remaining Work

- Audit TODO/18 requirement-by-requirement against current code and pushed
  evidence before calling Level 2 complete.
- Decide whether opaque JSON evidence/metadata should remain a documented Level
  2 exception or need one final typed-vocabulary cleanup.
- Keep public API DTOs HTTP-owned and separate from application commands and
  outputs.
- Keep use cases as the real authorization guard; middleware remains early
  rejection.
- Do not jump to Level 3 crates/microservices or abstractions that only move
  files around.
- Rough remaining effort: final requirement audit, plus a small cleanup only if
  the audit proves one of the remaining exception candidates is a real leak.
