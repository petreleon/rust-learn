---
name: automatic-tester
description: "Automated frontend, backend, and runtime test creation/execution for RustLearn or similar Rust/Next.js apps: Rust contract tests, API helper tests, route guard tests, Playwright/e2e checks, Docker Compose and Kubernetes smoke/log verification. Use when asked to add, run, debug, or harden repeatable tests."
---

# Automatic Tester

## Core Posture

Turn important product, API, and runtime behavior into repeatable proof. Do not
rely on a build or a health check when the risk is user-flow, permission, data,
or runtime behavior.

When working in RustLearn, read `TODO/11-qa-matrix.md`,
`TODO/05-testing-migration.md`, and the relevant milestone file before adding
tests.

## Workflow

1. Identify the behavior under test, required data, route, permission scope, and
   runtime surface.
2. Prefer the repo's existing package manager, scripts, Makefile targets, and
   test style.
3. Add the narrowest durable test first: Rust unit/integration test, frontend
   API helper test, route guard test, component behavior test, Playwright flow,
   Compose smoke, or Kubernetes smoke.
4. Cover failure modes that users actually hit: `401`, `403`, `404`, `409`,
   plain text errors, CSV success, timeout, network failure, and backend `5xx`.
5. Use stable selectors and accessible names. Avoid brittle screenshots as the
   only assertion.
6. Run the new test and the smallest relevant existing gate. Escalate to
   `make preflight`, Docker Compose, or Kubernetes checks when the blast radius
   warrants it.
7. Report exact commands, pass/fail result, and any untested residual risk.

## RustLearn Test Priorities

- Frontend API helper parsing for JSON, CSV, text errors, aborts, and timeouts.
- Backend contract tests for permission checks, response shapes, conflicts,
  idempotency, and state transitions.
- Current-user and route-guard behavior for multi-scope permissions.
- Product route smoke checks that replace operations-console smoke text.
- Docker Compose checks for web proxy, app readiness, worker health, RustFS/S3,
  and Anvil when touched.
- Kubernetes checks for rollout, pod readiness, in-cluster web-to-api readiness,
  product-route render, and recent log scans.

## Avoid

- Do not add broad flaky e2e tests when an API helper or route guard test proves
  the risk more cleanly.
- Do not mark a flow covered if only `/healthz` or static smoke text was tested.
- Do not hide a failing assertion by loosening it without explaining the product
  behavior.
- Use `$backend-dev` when repeatable coverage exposes an API contract or service
  bug.
