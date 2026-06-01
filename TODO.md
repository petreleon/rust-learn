# RustLearn TODO

This checklist is organized around the current RustLearn architecture: Actix Web API, Diesel/PostgreSQL persistence, S3-compatible storage, a background worker, Ethereum contracts, and a Next.js frontend.

## Priority 0 — Security, correctness, and contributor safety

- [x] Replace example RSA private/public keys in `.env.example` with non-secret placeholders and document local key generation only.
- [ ] Audit all API read endpoints for missing authorization checks, especially user, course, organization, role, wallet, transaction, notification, and reporting data.
- [x] Add password strength validation to registration.
- [ ] Add email confirmation or account-verification flow after registration.
- [x] Review JWT expiration, refresh/session strategy, and logout/revocation expectations.
- [ ] Add a JWKS or `.well-known/jwks.json` endpoint for external JWT verification.
- [ ] Confirm every role assignment path enforces hierarchy and scope constraints.
- [ ] Add structured logging for authentication failures, permission denials, worker failures, and blockchain operations.

## Priority 1 — Tests and quality gates

- [ ] Add API integration tests for `POST /api/auth/login` and `POST /api/auth/register`.
- [ ] Add endpoint-level authorization tests for user, course, organization, and role scopes.
- [ ] Add unit tests for authentication helper logic and edge cases.
- [ ] Add worker tests for retry state transitions, terminal failures, and heartbeat behavior.
- [ ] Keep blockchain integration tests covering deploy, mint, presigner, and EIP-2612 permit behavior.
- [ ] Add CI steps for `cargo fmt --all --check`, `cargo test`, and frontend lint/build checks.
- [ ] Document any tests that require Docker, PostgreSQL, Anvil/Geth, RustFS, or ffmpeg.

## Priority 2 — Developer experience and setup

- [x] Add a setup script to generate RSA keys and create a safe local `.env` from placeholders.
- [ ] Improve `.env.example` comments for PostgreSQL, S3/RustFS, Ethereum provider, admin bootstrap, and worker variables.
- [ ] Add a quickstart path for running only the API dependencies with Docker Compose.
- [ ] Add troubleshooting notes for Diesel migration failures, S3 connectivity, Ethereum RPC startup, and worker memory limits.
- [ ] Keep `Makefile` targets aligned with README examples.
- [ ] Add a short architecture diagram or request-flow diagram to the docs.

## Priority 3 — Product features

- [ ] Define the complete learning reward lifecycle: course event, eligibility check, reward calculation, token mint/transfer, wallet credit, notification, and audit record.
- [ ] Implement event listeners or reconciliation jobs for token mint/transfer events.
- [ ] Complete wallet-linking flows and end-to-end tests.
- [ ] Expand notifications for enrollment, content publication, role assignment, worker failures, and reward events.
- [ ] Build learner and administrator workflows in the Next.js frontend.
- [ ] Add course search/filtering, pagination, and organization-specific course discovery.
- [ ] Add reporting/export workflows for organizations and platform administrators.

## Priority 4 — Operations and deployment

- [ ] Verify Docker Compose builds on Linux and ARM64 targets.
- [ ] Validate Kubernetes manifests against the current service names, health checks, ports, and environment variables.
- [ ] Add production-oriented health/readiness endpoints for API dependencies.
- [ ] Add worker metrics for queue depth, attempts, processing duration, and failed jobs.
- [ ] Define backup/restore expectations for PostgreSQL, S3 objects, and blockchain-related persistent state.
- [ ] Review release process for Ethereum contract artifact generation and deployment addresses.

## Completed foundation

- [x] Actix Web API with route scopes for authentication, users, courses, organizations, and roles.
- [x] Diesel/PostgreSQL models, migrations, and async connection pooling.
- [x] Platform, organization, and course role/permission models with hierarchy-aware middleware infrastructure.
- [x] Default role assignment on user registration.
- [x] Role listing and role assignment endpoints.
- [x] S3-compatible object storage integration through AWS SDK and RustFS-compatible configuration.
- [x] Background worker binary for upload/video processing with retry configuration and heartbeat health check.
- [x] LearnToken Solidity contracts, generated artifacts, startup deployment support, and blockchain integration test coverage.
- [x] Docker Compose, Dockerfiles, Kubernetes manifests, and Makefile commands for common development/deployment tasks.
- [x] Permission matrix documentation in `PERMISSIONS.md`.
