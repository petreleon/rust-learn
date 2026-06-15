# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base: `87c4ca4b` (`Move reward JSON vocabulary into domain`).

Goal: finish RustLearn as a Level 2 modular monolith. Keep the main rings
`domain`, `application`, `infra`, `http`, and `bootstrap`; use granular
submodules only where a context or use case has real ownership.

## Rules

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
- Batch ready non-conflicting fixes by architectural concern; defer ambiguous,
  risky, behavior-changing, or unrelated items.

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
- `7693fb71`: abstract readiness dependency labels plus wallet/platform reward
  application command/output names separated from HTTP DTO names.
- `87c4ca4b`: reward candidate evidence and reward audit metadata moved into
  domain reward vocabulary; unused top-level `shared` module removed.

Latest pushed proof for `87c4ca4b`:

- `cargo fmt --all --check`.
- Focused tests: `evidence`, `--test reward_candidates`,
  `--test reward_candidate_audit`, `--test reward_course_candidates`, and
  `--test api_routing`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`, no `shared`/`JsonValue` references, no top-level
  `src/shared`, domain/application/http boundary scans, no `include!`, no
  `imports.rs`, and maintained Rust line-count scan.

## Current Verified Batch

Status: verified locally, pending commit/push.

Included problem:

- Diesel records lived in top-level `src/models`, outside the Level 2 rings,
  while every real consumer was infra, worker/runtime code, model-internal
  association metadata, or DB-backed tests.

Fixes:

- Moved the 47 Diesel record files to `src/infra/postgres/models`.
- Exposed records through `infra::postgres::models`.
- Removed root `models` modules from `src/lib.rs` and `src/main.rs`.
- Updated infra, worker, model-internal Diesel associations, and DB-backed tests
  from `crate::models` / `rust_learn::models` to the Postgres infra path.

Deferred problems:

- Top-level `src/db` still owns the Diesel schema and `DbPool`. Moving schema
  and pool construction touches generated Diesel schema, bootstrap wiring, and
  test setup, so it should be audited separately instead of mixed with the
  record-location move.
- Remaining application `Request` strings are business concepts or verbs
  (`RequestPasswordResetCommand`, `RequestUploadUrlCommand`,
  `RequestMediaUrlCommand`, course join-request outputs). They remain deferred
  to a broader product-language audit.

Current batch proof:

- Discovery scans: no domain/application/http/bootstrap imports of
  `models`; consumers were infra, one worker module, model-internal references,
  and DB-backed tests.
- `cargo fmt --all --check`.
- Focused tests: `--test repository_core_tests`,
  `--test repository_reward_tests`, `--test worker_upload_jobs`,
  `--test current_session_api`, and `--test api_routing`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`.
- Scans: no code references to the old root model path, no top-level
  `src/models`, `src/infra/postgres/models` present, no domain/application
  infra/HTTP/DB leaks, no HTTP DB leaks, no application DTO types, no
  `include!`, no `imports.rs`, and no maintained Rust file over 180 lines.

## Remaining Actionable Checklist

- Audit TODO/18 requirement-by-requirement against current code and pushed
  evidence before calling Level 2 complete.
- Audit top-level `src/db` schema/pool ownership and decide whether a focused
  `infra/postgres` move is safe or whether generated schema should remain a
  documented exception.
- Continue raw-vocabulary scans for migrated contexts and only batch fixes that
  share a reviewable architectural concern.
- Keep public API DTOs HTTP-owned and separate from application outputs.
- Keep use cases as the real authorization guard; middleware remains early
  rejection.
- Do not jump to Level 3 crates/microservices or abstractions that only move
  files around.
