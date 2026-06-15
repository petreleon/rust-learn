# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified pushed base: `d5a5172e` (`Type reward command vocabulary`).
Current verified changeset: teacher-application audit event vocabulary in this
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

## Verification Proof

Latest pushed batch (`d5a5172e`) passed:

- Focused host tests for reward policy create/list, submit-candidate behavior,
  course reward policy validation, organization/delegated reward submission,
  evidence threshold checks, and teacher course dashboard reward progress.
- `cargo fmt --all --check`.
- `./scripts/run-host-tests.sh cargo check --lib`.
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`.
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- `git diff --check`, dependency boundary scans, no `include!`, no
  `tests/*/imports.rs`, targeted raw-vocabulary scans for touched application
  paths, and manual line-count scan.

Current verified changeset passed:

- `TeacherApplicationAuditEventType` now owns submitted, organization-nominated,
  approved, needs-changes, and rejected audit-event vocabulary in domain.
- Teacher-application audit outputs, notification commands, platform-review
  audit summaries, and organization teacher-application audit summaries now use
  typed event vocabulary in application.
- Postgres adapters parse persisted audit event strings at infra boundaries and
  persist typed event values back to strings.
- HTTP DTOs still expose public JSON strings and convert typed application
  values at the HTTP boundary.
- Passed `./scripts/run-host-tests.sh cargo test teacher_application`.
- Passed `cargo fmt --all --check`.
- Passed `./scripts/run-host-tests.sh cargo check --lib`.
- Passed `./scripts/run-host-tests.sh cargo check --bin rust-learn --features
  app-bin`.
- Passed `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`.
- Passed `git diff --check`, strict domain/application/http boundary scans, no
  `include!`, no `imports.rs`, targeted raw teacher-application event scans, and
  manual Rust line-count scan.

## Still Open

- Replace remaining raw application event vocabulary:
  wallet audit/history external transaction event type, KYC audit output/facts,
  reporting token payout CSV event type, reward history external transaction
  event summaries, and organization member audit events.
- Keep Diesel schema/model leakage inside infra records.
- Keep public API DTOs HTTP-owned and separate from application outputs.
- Keep use cases as the real authorization guard; middleware remains early
  rejection.
- Do not jump to Level 3 crates/microservices or abstractions that only move
  files around.
