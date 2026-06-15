# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base: `004d1037` (`Type audit event vocabulary`).

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

Latest pushed proof for `004d1037`:

- Focused tests: `audit_event_types`, `list_organization_member_audit`,
  `domain::kyc::submission`, `--test kyc_review`,
  `--test organization_members`, `--test api_routing`, and
  `--test teacher_applications platform_teacher_application_review_contract_returns_context_and_filters`.
- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`, targeted raw event scans, targeted
  domain/application/http boundary scans, no `include!`, no `imports.rs`, and
  maintained Rust line-count scan.

## Current Verified Batch

Status: verified locally, pending commit/push.

Included problems:

- Operations readiness exposed concrete dependency labels (`postgres`, `s3`,
  `ethereum`) at application/HTTP boundaries.
- Wallet application commands still used HTTP-shaped `Request` names.
- Platform reward candidate application output still used an HTTP-shaped
  `Response` name.

Fixes:

- Added application readiness dependency vocabulary:
  `READINESS_DEPENDENCY_DATABASE`, `READINESS_DEPENDENCY_OBJECT_STORAGE`, and
  `READINESS_DEPENDENCY_BLOCKCHAIN`.
- Infra adapters map Postgres, object storage, and Ethereum checks into those
  abstract labels.
- HTTP readiness fallback now reports `database`, `object_storage`, and
  `blockchain`.
- Renamed application types to `WalletDepositIntentCommand`,
  `WalletRetirementCommand`, and `PlatformRewardCandidatesOutput`.
- Kept HTTP DTO ownership in HTTP:
  `WalletDepositIntentRequestDto`, `WalletRetirementRequestDto`, and
  `PlatformRewardCandidatesResponseBody`.

Deferred problems:

- Remaining application `Request` strings are business concepts or verbs
  (`RequestPasswordResetCommand`, `RequestUploadUrlCommand`,
  `RequestMediaUrlCommand`, course join-request outputs). They are not safe to
  batch with DTO cleanup without a broader product-language audit.
- Full runtime readiness test still depends on live object storage and timed
  out once, so the deterministic missing-use-case readiness test is counted as
  proof instead.

Current batch proof:

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
- `git diff --check`.
- Scans: no old application names, no concrete readiness labels in the touched
  operations boundary, no domain/application external imports, no HTTP DB leaks,
  no application DTO/response-body types, no `include!`, no `imports.rs`, and
  no maintained Rust file over 180 lines.

## Remaining Actionable Checklist

- Audit TODO/18 requirement-by-requirement against current code and pushed
  evidence before calling Level 2 complete.
- Continue raw-vocabulary scans for other migrated contexts and only batch
  fixes that share a reviewable architectural concern.
- Keep Diesel schema/model leakage inside infra records.
- Keep public API DTOs HTTP-owned and separate from application outputs.
- Keep use cases as the real authorization guard; middleware remains early
  rejection.
- Do not jump to Level 3 crates/microservices or abstractions that only move
  files around.
