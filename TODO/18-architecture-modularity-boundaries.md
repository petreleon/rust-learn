# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base: `7693fb71` (`Clarify readiness and DTO boundaries`).

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

Latest pushed proof for `7693fb71`:

- `cargo fmt --all --check`.
- Focused tests:
  `create_deposit_intent`, `retire_tokens`,
  `enriches_searches_and_pages_platform_candidates`,
  `rejects_without_platform_audit_permission`,
  `readiness_reports_not_ready_without_configured_use_case`,
  `--test api_routing`, and
  `--test wallet_linking duplicate_pending_deposit_intents_are_marked_ambiguous_without_crediting`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`, old-name scans, concrete readiness-label scans,
  domain/application/http boundary scans, no `include!`, no `imports.rs`, and
  maintained Rust line-count scan.

## Current Verified Batch

Status: verified locally, pending commit/push.

Included problem:

- `src/shared` was a sixth top-level module outside the Level 2 rings. Its only
  live type, `JsonValue`, carried reward candidate evidence and reward audit
  metadata across domain/application/http without domain ownership.

Fixes:

- Added `RewardEvidence` to `domain::rewards::candidate::evidence`.
- Added `RewardAuditMetadata` to `domain::rewards::audit`.
- Updated reward application outputs/commands and reward HTTP DTOs to use the
  domain reward vocabulary.
- Removed `pub mod shared` and deleted the unused placeholder shared modules.

Deferred problems:

- Top-level `src/models` still exists as the Diesel record/model surface used by
  infra and DB-backed tests. Moving it under infra would be much larger,
  mechanically noisy, and should be audited as its own batch.
- Remaining application `Request` strings are business concepts or verbs
  (`RequestPasswordResetCommand`, `RequestUploadUrlCommand`,
  `RequestMediaUrlCommand`, course join-request outputs). They remain deferred
  to a broader product-language audit.

Current batch proof:

- Discovery scans: no `src/services` or `src/repositories`; `src/shared` only
  contained placeholders plus `JsonValue`; `JsonValue` was used only in reward
  evidence/metadata paths.
- `cargo fmt --all --check`.
- Focused tests: `evidence`, `--test reward_candidates`,
  `--test reward_candidate_audit`, `--test reward_course_candidates`, and
  `--test api_routing`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`.
- Scans: no `shared`/`JsonValue` references, no top-level `src/shared`, no
  domain/application infra/HTTP/DB leaks, no HTTP DB leaks, no application DTO
  types, no `include!`, no `imports.rs`, and no maintained Rust file over 180
  lines.

## Remaining Actionable Checklist

- Audit TODO/18 requirement-by-requirement against current code and pushed
  evidence before calling Level 2 complete.
- Audit top-level `src/models` and decide whether to keep it as legacy Diesel
  records or move record ownership under `infra/postgres` in focused batches.
- Continue raw-vocabulary scans for migrated contexts and only batch fixes that
  share a reviewable architectural concern.
- Keep public API DTOs HTTP-owned and separate from application outputs.
- Keep use cases as the real authorization guard; middleware remains early
  rejection.
- Do not jump to Level 3 crates/microservices or abstractions that only move
  files around.
