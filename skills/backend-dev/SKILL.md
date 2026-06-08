---
name: backend-dev
description: "Implement RustLearn backend/API work in Rust Actix Web with Diesel/PostgreSQL, repository/service layers, permission-scoped authorization, migrations, workers, wallets, reports, and frontend contract endpoints. Use when Codex is asked to add, fix, or refactor backend contracts, handlers, services, repositories, migrations, or Rust tests."
---

# Backend Dev

## Core Posture

Build backend contracts that product routes can trust. Keep business logic out
of Actix handlers, keep authorization permission-scoped, and verify behavior
through the narrowest meaningful Rust, Docker Compose, and Kubernetes checks.

When working in RustLearn, read `AGENTS.md`, the relevant `TODO/` contract or
milestone file, and nearby API/service/repository tests before editing code.

## Workflow

1. Identify the frontend or API contract: route, caller, payload, response,
   errors, permissions, and runtime dependencies.
2. Locate existing patterns in `src/api/`, `src/services/`, `src/repositories/`,
   `src/models/`, `migrations/`, and `tests/`.
3. Put database access in repositories and business decisions in services.
   Keep handlers thin: parse request, extract auth, call service, map response.
4. Enforce platform, organization, course, or delegated permissions explicitly.
   Do not authorize business behavior by role name.
5. Add reversible migrations and update `src/db/schema.rs` when schema changes
   require it.
6. Normalize response contracts when product frontend work needs predictable
   JSON, text, CSV, pagination, status, or error behavior.
7. Add focused tests for permissions, state transitions, idempotency, conflict,
   not found, invalid input, and retry-safe behavior.
8. Run the smallest relevant checks first, then escalate to `make preflight`,
   `make test-compose`, or runtime verification when the blast radius warrants.

## RustLearn Contract Priorities

- Current-user/session endpoint for frontend route guards.
- Course discovery/detail payloads for learner product routes.
- Learner progress, content completion, and notification contracts.
- Upload-job and media-processing status contracts for teacher routes.
- Searchable user, organization, course, application, reward, delegation, and
  fraud-block lookups so product UI does not require raw ids.
- Pagination and filter metadata for queues, reports, audit logs, dashboards.
- Wallet operation states for MetaMask-required, platform-gas, tax,
  insufficient funds, pending deposit, retirement, and reconciliation cases.

## Runtime Guardrails

- Use `./scripts/run-host-tests.sh cargo test ...` for direct host Cargo-style
  checks that need local service-name rewrites.
- Use `make test` for host Rust tests and `make test-compose` for Compose
  network integration.
- Run blockchain tests when Ethereum contracts, token execution, wallet
  deposits, or reconciliation behavior changes.
- Run worker and log checks when upload jobs, wallet indexer, video processing,
  or background retries change.
- Update `.env.example`, `README.md`, `TODO/`, and operational docs when
  behavior, setup, permissions, or environment variables change.

## Handoffs

- Use `$frontend-dev` when a backend contract is ready for product UI.
- Use `$automatic-tester` when adding regression coverage or runtime gates.
- Use `$product-owner` when the response contract or acceptance criteria is
  ambiguous.
- Use `$product-manager` when a backend contract changes milestone sequencing.

