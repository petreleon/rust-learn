# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified base before this changeset: `ce829ddb`
(`Type token event boundaries`).
Current changeset: reward policy/candidate command vocabulary in this commit,
verified locally.

Goal: finish RustLearn as a Level 2 modular monolith. Keep the main rings:
`domain`, `application`, `infra`, `http`, `bootstrap`. Use granular modules
inside those rings when a context or use case has real ownership.

## Boundary Contract

- `http -> application -> domain`; `infra` implements application ports;
  `bootstrap` wires concrete state.
- `domain`: pure vocabulary, invariants, transitions.
- `application`: commands, use cases, outputs, errors, ports, authorization
  decisions, orchestration.
- `infra`: Postgres, object storage, Ethereum, email, workers, dispatchers.
- `http`: Actix routes, extractors, DTOs, request/response mapping.
- Forbidden in `domain`/`application`: Actix, Diesel, S3, Ethereum, env vars,
  HTTP responses, request extractors, concrete pools, and web state.

## Done

- Migrated contexts checked: `access_control`, `content`, `identity`, `kyc`,
  `learning`, `notifications`, `operations`, `organizations`, `reporting`,
  `rewards`, `teacher_applications`, `wallet`.
- Bootstrap is thin; migrated HTTP owns DTOs/extractors/errors and delegates to
  application use cases.
- Migrated use cases use commands, outputs, ports, fake-port tests, typed access
  decisions, domain vocabulary, invariants, and transition helpers.
- Infra owns concrete adapters; Diesel stays out of migrated route handlers and
  migrated application/domain code.
- Shared auth/access-control vocabulary, typed auth extractors, frontend
  capability gates, and `/api/me` unauthorized JSON contract are checked.
- Manual Rust file cap is currently clean except generated/exempt files.

## Verified Commits

- `581f1be3`..`57215a3c`: auth helper removal, JSON extractor errors, typed
  HTTP results/errors, access decisions, frontend gates, test harness cleanup.
- `5a1643fa`..`b1413b5d`: reward payout/read/audit/reconciliation/source/event,
  payment, policy, fraud, and token-confirmation vocabulary.
- `53a3fd67`..`d1ca7530`: platform reward dashboards/exports, teacher and
  delegated-permission CSV vocabulary, fraud dashboard scope, wallet owner
  outputs.
- `0db0000c`: delegated-permission management scope vocabulary across
  application, Postgres, HTTP, and routing fakes.
- `e4ff6767`: wallet deposit status vocabulary across domain, application,
  Postgres, HTTP, and integration helpers.
- `782a5a3c`: wallet reward-audit candidate and reconciliation status
  vocabulary across domain, application, Postgres, HTTP, and `wallet_audit`.
- `ce829ddb`: wallet deposit event type, reward token confirmation event type,
  and token reconciliation event type are typed across domain, application,
  Ethereum/Postgres adapters, HTTP/test helpers, and wallet/reward integration
  tests.

## Current Verified Changeset

- `CreateRewardPolicyCommand` and `ListRewardPoliciesQuery` now carry typed
  reward policy scope/event/payment vocabulary; HTTP parses request strings
  before calling application use cases.
- `SubmitRewardCandidateCommand` now carries typed `RewardEventType`; infra no
  longer reparses reward event strings before persistence/eligibility checks.
- Teacher-student reward latest-candidate output now carries typed
  `RewardEventType`, with Postgres parsing DB strings at the adapter boundary
  and HTTP mapping back to DTO strings.

Proof for this changeset:

- Passed focused host tests for submit-candidate handler behavior, reward policy
  create/list flows, course reward policy validation, organization/delegated
  reward candidate submission, evidence threshold checks, and teacher course
  dashboard reward progress.
- Passed `cargo fmt --all --check`.
- Passed `./scripts/run-host-tests.sh cargo check --lib`.
- Passed `./scripts/run-host-tests.sh cargo check --bin rust-learn --features
  app-bin`.
- Passed `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- Passed `git diff --check`, dependency boundary scans, no `include!`, no
  `tests/*/imports.rs`, targeted raw vocabulary scans for touched application
  paths, and manual line-count scan.

## Still Open

- Replace remaining raw application event vocabulary: wallet audit/history
  external transaction event type, teacher-application audit/notification and
  latest-event summaries, KYC audit output/facts, reporting token payout CSV
  event type, organization teacher-application latest-event summaries, and
  organization member audit events.
- Keep Diesel schema/model leakage inside infra records.
- Keep public API DTOs HTTP-owned and separate from application outputs.
- Keep use cases as the real authorization guard; middleware remains early
  rejection.
- Do not jump to Level 3 crates/microservices or abstractions that only move
  files around.
