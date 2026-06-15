# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base: `259df7c5` (`Type teacher application event vocabulary`).
Current verified changeset: external token transaction event vocabulary in this
commit, verified locally.

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

## Completed / Checked

- Migrated contexts checked: `access_control`, `content`, `identity`, `kyc`,
  `learning`, `notifications`, `operations`, `organizations`, `reporting`,
  `rewards`, `teacher_applications`, `wallet`.
- Bootstrap is thin; migrated HTTP owns DTOs, extractors, errors, and request
  mapping before delegating to application use cases.
- Migrated application code owns commands, outputs, ports, errors,
  authorization decisions, orchestration, and fake-port tests.
- Domain owns typed vocabulary, invariants, and transition helpers for migrated
  boundaries; infra owns Diesel/Postgres/external adapters.
- Shared auth/access-control vocabulary, typed auth extractors, frontend gates,
  and `/api/me` unauthorized JSON behavior are checked.
- Manual Rust file cap is clean for maintained non-Markdown files; generated,
  lockfile, binary, and tool-owned artifacts are exempt.

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
- `d5a5172e`: reward policy command/query vocabulary, reward-candidate command
  event vocabulary, and teacher-student reward latest-candidate event vocabulary
  are typed across domain/application/infra/http.
- `259df7c5`: teacher-application audit/notification/latest-event vocabulary
  is typed across domain/application/infra/http and organization
  teacher-application summaries.

## Verification Proof

Latest pushed batch (`259df7c5`) passed:

- Focused host tests for teacher-application domain/application behavior,
  platform/organization review flows, notification commands, and integration
  flows.
- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`, dependency boundary scans, no `include!`, no
  `tests/*/imports.rs`, targeted raw-vocabulary scans for touched application
  paths, and manual line-count scan.

Current verified changeset passed:

- `RewardTokenEventType` now normalizes external token event strings and is used
  for wallet audit external transactions, student reward-history token
  transactions, and platform token-payout CSV export rows.
- Postgres wallet, reward-history, and reporting adapters parse persisted
  optional external transaction event strings before creating application
  outputs.
- HTTP and CSV DTOs still expose public strings and convert typed application
  values at the presentation boundary.
- Token vocabulary tests moved to a small sibling module so maintained Rust
  files stay under the 180-line cap.
- Passed `./scripts/run-host-tests.sh cargo test token`.
- Passed `./scripts/run-host-tests.sh cargo test student_reward_history`.
- Passed `./scripts/run-host-tests.sh cargo test wallet_audit`.
- Passed `./scripts/run-host-tests.sh cargo test --test reporting_exports`.
- Passed `cargo fmt --all --check`.
- Passed `./scripts/run-host-tests.sh cargo check --lib`.
- Passed `./scripts/run-host-tests.sh cargo check --bin rust-learn --features
  app-bin`.
- Passed `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- Passed `git diff --check`, strict domain/application/http boundary scans, no
  `include!`, no `imports.rs`, targeted raw external-token event scans, and
  manual Rust line-count scan.

## Still Open

- Replace remaining raw application event vocabulary:
  KYC audit output/facts and organization member audit events.
- Keep Diesel schema/model leakage inside infra records.
- Keep public API DTOs HTTP-owned and separate from application outputs.
- Keep use cases as the real authorization guard; middleware remains early
  rejection.
- Do not jump to Level 3 crates/microservices or abstractions that only move
  files around.
