# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-16.
Latest verified pushed base before current batch: `0fa7a359`
(`Name opaque JSON payload boundaries`).

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
- `cc74255c`: delegated permission normalization and scope rules now use the
  canonical `Permissions` catalog; raw permission-name strings remain only at
  public/persisted boundaries and tests asserting that contract.
- `c38b13fe`: Make targets are the primary documented development command
  surface, Diesel development commands run through the Compose tool container,
  `make web-dev` exists for the frontend dev loop, and active docs point at
  `src/infra/postgres/schema.rs`.
- `0fa7a359`: opaque JSON product payloads are named at application boundaries
  with domain vocabulary (`RewardEvidence`, `KycAuditMetadata`,
  `TeacherApplicationPortfolioLinks`), public/DB JSON shapes are preserved, and
  the remaining oversized integration tests were trimmed to 180 lines.

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

## Completion Audit

Audit questions:

- Do the main rings match Level 2 ownership?
- Are `domain` and `application` free of Actix, Diesel/Postgres, concrete infra,
  env, filesystem, network, and web-state leaks?
- Does `http` own Actix routes, DTOs, and HTTP error mapping without importing
  infra directly?
- Are public API DTOs separate from application commands/outputs?
- Do use cases remain the real authorization guard?
- Are old root-level DB/model/config/middleware modules gone?
- Are opaque JSON payloads intentional domain-named product payloads rather than
  HTTP DTO leakage?
- Are manually maintained non-Markdown files within the 180-line rule?
- Did the migration avoid Level 3 crates/microservices or abstraction churn?

Conclusion:

- Level 2 is complete in the current code. No remaining code or documentation
  changes are required by this TODO.

Deferred problems:

- None.

Proof:

- `git status --short --branch` was clean at pushed base `0fa7a359`.
- Ring layout is present under `src/domain`, `src/application`, `src/infra`,
  `src/http`, and `src/bootstrap`; `Cargo.toml` remains a single crate with
  binaries, not a Level 3 workspace split.
- `src/db`, `src/models`, `src/config`, and `src/middlewares` are absent.
- Source scans find no old root imports such as `crate::db`,
  `rust_learn::models`, `crate::config`, or `crate::middlewares`.
- Domain/application forbidden-dependency scan finds no concrete dependency
  leaks; the only hit is the permission vocabulary variant `MANAGE_S3_OBJECTS`.
- HTTP infra scan finds no `crate::infra`, Diesel/Postgres, S3, Ethereum, env,
  or reqwest imports in `src/http`.
- DTO/Actix scan finds no Actix/HTTP DTO ownership in `src/application` or
  `src/domain`; the only serde derive in domain is `UserJWT`, the token-claim
  vocabulary used by the infra token verifier.
- Production application JSON scan finds no raw `serde_json::Value` payload
  fields or constructors; opaque JSON remains only as domain-named payload
  aliases and HTTP/infra serialization details.
- Line-count scan over maintained source, test, Make, script, Docker, and K8s
  files finds no non-generated file above 180 lines.
- Current pushed verification includes formatting, focused tests for touched
  contexts, library check, app binary check, and `cargo test --tests --no-run`.

## Remaining Work

- None for TODO/18.
