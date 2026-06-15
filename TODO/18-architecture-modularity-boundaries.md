# TODO 18: Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-15.
Latest verified commit before this changeset: `d1ca7530`.
Current verified changeset: delegated-permission management scope vocabulary
(commit pending).

Goal: make RustLearn a Level 2 modular monolith with firm boundaries.
Ownership matters more than folder count; preserve behavior unless another TODO
owns the behavior change.

## Boundary Rules

- `http -> application -> domain`; `infra` implements ports; `bootstrap` wires.
- `domain`: pure vocabulary, invariants, transitions.
- `application`: commands, use cases, outputs, errors, ports.
- `infra`: Postgres, object storage, Ethereum, email, workers, dispatchers.
- `http`: Actix routes, extractors, DTOs, response mapping.
- `shared`: tiny cross-context primitives only.
- Forbidden in `domain`/`application`: Actix, Diesel, S3, Ethereum, env vars,
  HTTP responses, request extractors, concrete pools, and web state.

## Done / Checked

- Checked contexts: `access_control`, `content`, `identity`, `kyc`,
  `learning`, `notifications`, `operations`, `organizations`, `reporting`,
  `rewards`, `teacher_applications`, `wallet`.
- Bootstrap is thin; migrated HTTP uses typed extractors, DTOs, errors, and
  use cases.
- Migrated workflows use commands, outputs, ports, fake-port tests, access
  decisions, domain vocabulary, invariants, and transition helpers.
- Infra owns concrete adapters; Diesel is kept out of route handlers and
  migrated application/domain code.
- Shared access-control vocabulary, typed auth extractors, frontend capability
  gates, and `/api/me` unauthorized JSON contract are checked.
- Boundary scans checked: no legacy `src/{api,services,repositories,utils}`, no
  `include!`, no `tests/*/imports.rs`, no direct auth/pool access in `http`,
  and no forbidden ring imports in `domain`/`application`.

## Verified Commit Groups

- `581f1be3`..`57215a3c`: auth helper removal, JSON extractor errors, typed
  HTTP results/errors, access decisions, frontend gates, test harness cleanup.
- `5a1643fa`..`b1413b5d`: reward payout/read/audit/reconciliation/source/event,
  payment, policy, fraud, and token-confirmation vocabulary.
- `53a3fd67`..`d1ca7530`: platform reward dashboard rows, reward approval CSV,
  teacher application CSV, delegated-permission CSV, fraud dashboard scope, and
  wallet owner-type outputs.
- Commit pending: delegated-permission management uses typed scope vocabulary
  across application outputs/store/filter, Postgres adapters, HTTP DTO mapping,
  and routing fakes.

Proof set used across verified batches: focused host tests, `cargo fmt`,
Cargo lib/bin checks, integration no-run compile, `git diff --check`, line
counts, string-field scans, and boundary scans. This changeset additionally
passed `cargo test delegated_permission`.

## Still Open

- Continue replacing raw app/infra string vocabulary at remaining
  reward/reporting/wallet/audit surfaces.
- Keep Diesel schema/model leakage inside infra records.
- Keep public API DTOs HTTP-owned and separate from application outputs.
- Keep use cases as the real authorization guard; middleware is early
  rejection.
- No Level 3 crates/microservices, all-at-once rewrite, or abstractions that
  only move files around.
