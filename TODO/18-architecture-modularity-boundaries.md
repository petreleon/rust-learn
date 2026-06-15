# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base before current batch: `380f9424`
(`Use catalog permissions across application handlers`).

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

- `src/http/middlewares/jwt_middleware.rs` imported
  `infra::tokens::jwt::decode_jwt` directly, so HTTP knew the concrete token
  adapter and JWT error source.
- Direct JWT middleware test apps could wrap middleware without registering an
  auth verifier because the middleware reached into infra itself.
- Diesel development already used Compose, but schema refresh was not a
  first-class Make target and migration targets relied on implicit schema
  behavior instead of an explicit `make schema` step.

Fixes:

- Added `application::identity::auth_token` with `AuthTokenVerifier`,
  `AuthTokenVerifierService`, and `AuthTokenVerificationError`.
- Added `infra::tokens::jwt::EnvAuthTokenVerifier`, which implements the
  application port using the existing environment-backed JWT decode logic.
- Bootstrap now owns/registers the auth verifier app data; direct test apps use
  `auth_token_verifier_app_data()`.
- HTTP JWT middleware now parses the header, resolves the application verifier,
  maps application token errors to 401 responses, and inserts `UserJWT` without
  importing infra.
- Added `make schema`, `DIESEL_COMPOSE_RUN`, and `DIESEL_SCHEMA_FILE`; `make
  migrate` and `make migrate-redo` now run Compose Diesel migrations and then
  refresh `src/infra/postgres/schema.rs` explicitly through Compose.
- The Diesel CLI Docker stage now installs `rustfmt`, so `make schema` formats
  generated schema output inside the tool container.
- README and AGENTS now state that Make is the primary development interface for
  Diesel work; host Diesel is not required.

Deferred problems:

- Integration tests still use infra JWT helpers where they mint or assert real
  token contracts. That is test fixture/support code, not a production
  `http -> infra` boundary leak.
- Domain delegation permission rules still normalize a scoped raw string policy
  list; audit separately if more vocabulary cleanup is needed.
- The smaller `domain::access_control::Permission` enum still overlaps with the
  full `Permissions` catalog; unification would be a deliberate authorization
  API cleanup.

Proof:

- `cargo fmt --all --check`.
- `docker compose -f docker-compose.yml -f docker-compose.tools.yml config
  --quiet`.
- `make -n schema`; `make -n migrate`; `make -n migrate-redo`; `make -n
  diesel-compose DIESEL_ARGS='migration list'`.
- `make diesel-compose DIESEL_ARGS='--version'`.
- `make schema` regenerated and formatted `src/infra/postgres/schema.rs`
  without leaving a schema diff.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo test --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features
  app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `./scripts/run-host-tests.sh cargo test --test current_session_api --test
  middleware_access_control --test authentication_flow`.
- `git diff --check`.
- Scans: no `crate::infra` / `rust_learn::infra` imports remain in `src/http`;
  domain/application boundary scan found no concrete dependency leaks, only the
  domain vocabulary variant `MANAGE_S3_OBJECTS`; no maintained Rust file over
  180 lines.

## Remaining Work

- Audit TODO/18 requirement-by-requirement against current code and pushed
  evidence before calling Level 2 complete.
- Decide whether the overlapping `Permission` and `Permissions` domain enums
  should stay separate or be unified through a deliberate authorization API
  cleanup.
- Keep public API DTOs HTTP-owned and separate from application commands and
  outputs.
- Keep use cases as the real authorization guard; middleware remains early
  rejection.
- Do not jump to Level 3 crates/microservices or abstractions that only move
  files around.
- Rough remaining effort: final requirement audit, plus one small authorization
  vocabulary decision only if the audit requires it.
