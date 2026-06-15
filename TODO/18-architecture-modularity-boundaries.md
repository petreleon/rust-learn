# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base: `28019ba3` (`Type external token event outputs`).
Current verified batch: KYC and organization member audit event vocabulary.

Goal: finish RustLearn as a Level 2 modular monolith. Keep the main rings
`domain`, `application`, `infra`, `http`, and `bootstrap`; use granular
submodules only where a context or use case has real ownership.

## Boundary Contract

- Flow: `http -> application -> domain`; `infra` implements application ports;
  `bootstrap` wires concrete state.
- `domain`: pure vocabulary, invariants, transition helpers.
- `application`: commands, outputs, errors, ports, authorization decisions,
  orchestration.
- `infra`: Postgres, object storage, Ethereum, email, workers, dispatchers.
- `http`: Actix routes, extractors, DTOs, request/response mapping.
- Forbidden in `domain`/`application`: Actix, Diesel, S3, Ethereum, env vars,
  HTTP responses, request extractors, concrete pools, and web state.

## Done / Checked

- Migrated contexts checked: `access_control`, `content`, `identity`, `kyc`,
  `learning`, `notifications`, `operations`, `organizations`, `reporting`,
  `rewards`, `teacher_applications`, and `wallet`.
- Bootstrap is thin; migrated HTTP owns DTOs, extractors, errors, and request
  mapping before delegating to application use cases.
- Migrated application code owns commands, outputs, ports, errors,
  authorization decisions, orchestration, and fake-port tests.
- Domain owns typed vocabulary, invariants, and transitions for migrated
  boundaries; infra owns Diesel/Postgres/external adapters.
- Shared auth/access-control vocabulary, typed auth extractors, frontend gates,
  and `/api/me` unauthorized JSON behavior are checked.
- Public HTTP/CSV shapes still expose strings where needed, but convert typed
  application values at the presentation boundary.
- Manual Rust file cap is clean for maintained non-Markdown files; generated,
  lockfile, binary, and tool-owned artifacts are exempt.

## Verified Typed Vocabulary Work

- Access/auth HTTP result and error boundaries.
- Reward payout/read/audit/reconciliation/source/event/payment/policy/fraud
  vocabulary.
- Platform reward dashboards/exports, teacher exports, delegated-permission
  CSV vocabulary, fraud dashboard scope, and wallet owner outputs.
- Delegated-permission management scope.
- Wallet deposit status, wallet reward-audit candidate status, reconciliation
  status, deposit event type, token-confirmation event type, token
  reconciliation event type, and external token transaction event type.
- Reward policy command/query vocabulary, reward-candidate command event
  vocabulary, teacher-student latest-candidate event vocabulary.
- Teacher-application audit, notification, latest-event, and organization
  teacher-application summary vocabulary.
- KYC audit event output/fact vocabulary and organization member audit event
  output/read/write vocabulary.

## Verified Commits

- `581f1be3`..`57215a3c`: auth helper removal, JSON extractor errors, typed
  HTTP results/errors, access decisions, frontend gates, and test harness
  cleanup.
- `5a1643fa`..`b1413b5d`: reward payout/read/audit/reconciliation/source/event,
  payment, policy, fraud, and token-confirmation vocabulary.
- `53a3fd67`..`d1ca7530`: platform reward dashboards/exports, teacher and
  delegated-permission CSV vocabulary, fraud dashboard scope, wallet owner
  outputs.
- `0db0000c`, `e4ff6767`, `782a5a3c`, `ce829ddb`, `d5a5172e`, `259df7c5`,
  `28019ba3`: delegated-permission scope, wallet statuses/events, token event
  boundaries, reward command vocabulary, teacher-application event vocabulary,
  and external token event outputs.

## Latest Verification Proof

Latest pushed batch (`28019ba3`) passed:

- Focused tests: `token`, `student_reward_history`, `wallet_audit`, and
  `--test reporting_exports`.
- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`, strict domain/application/http boundary scans, no
  `include!`, no `imports.rs`, targeted raw external-token event scans, and
  manual Rust line-count scan.

Current verified batch passed:

- Typed KYC audit events in `src/domain/kyc/audit.rs`,
  `src/application/kyc/output.rs`, Postgres KYC mappers/audit writes, and HTTP
  DTO conversion.
- Typed organization member audit events in
  `src/domain/organizations/member_audit.rs`, application outputs, Postgres
  member audit read/write adapters, and HTTP DTO conversion.
- Removed infra-shaped KYC domain-test fixture text and brought the last
  over-cap Rust test file back to 180 lines.
- Focused tests: `audit_event_types`, `list_organization_member_audit`,
  `domain::kyc::submission`, `--test kyc_review`, `--test organization_members`,
  `--test api_routing`, and
  `--test teacher_applications platform_teacher_application_review_contract_returns_context_and_filters`.
- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`, targeted raw application event scan, targeted
  domain/application/http boundary scans, no `include!`, no `imports.rs`, and
  manual Rust line-count scan.

## Still Open

- Audit TODO/18 requirement-by-requirement against current code and pushed
  evidence before calling the Level 2 migration complete.
- Keep Diesel schema/model leakage inside infra records.
- Keep public API DTOs HTTP-owned and separate from application outputs.
- Keep use cases as the real authorization guard; middleware remains early
  rejection.
- Do not jump to Level 3 crates/microservices or abstractions that only move
  files around.
