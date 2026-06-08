---
name: frontend-dev
description: "Implement product-grade frontend work for RustLearn or similar Next.js/React apps: app shell, auth/session UI, learner/teacher/organization/admin routes, permission-aware components, forms, data helpers, responsive layout, and UX refactors. Use when Codex is asked to build, refactor, or fix frontend code rather than only plan or test it."
---

# Frontend Dev

## Core Posture

Build the actual product UI, not a prettier API test harness. Prefer route-based
learner, teacher, organization, and platform admin experiences over console
controls. Keep authorization permission-driven even when the UI is organized by
persona.

When working in RustLearn, read `TODO/README.md`, then the relevant milestone
file and `TODO/00-self-criticism.md` before editing code.

## Workflow

1. Identify the user flow, persona, route, primary action, and required backend
   contract.
2. Check whether the backend contract exists. If it does not, use
   `$backend-dev` to implement it or record the gap before building fake UI
   behavior.
3. Implement with existing repo patterns first: Next.js app routes, React
   components, CSS modules, local helpers, and existing Makefile checks.
4. Build reusable components only when duplication or complexity is real.
5. Cover loading, empty, denied, validation, conflict, network failure, backend
   failure, success, and retry states where the flow can hit them.
6. Verify desktop and mobile rendering with Browser or Playwright. Check console
   health, no framework overlay, no horizontal overflow, and interaction proof.
7. Run relevant frontend checks, then Docker Compose or Kubernetes checks when
   the changed route is deployed there.

## RustLearn Guardrails

- Treat `web/src/app/page.tsx` as the current operations console until it is
  moved to `/ops`.
- Do not make normal users paste JWTs, type raw ids, or toggle permissions.
- Use resolved platform, organization, course, and delegated permissions for UI
  affordances.
- Keep product screens mobile-first for the first viewport and primary action.
- Preserve user input on validation, permission, network, and session failures
  where possible.
- Keep runtime verification honest: product routes must be tested, not only
  `/healthz` or operations-console smoke text.

## Handoffs

- Use `$product-owner` when value, acceptance, or scope is unclear.
- Use `$product-manager` when sequencing, dependencies, or roadmap updates are
  unclear.
- Use `$backend-dev` when a route needs new or changed API contracts,
  permissions, migrations, services, or repositories.
- Use `$manual-tester` for rendered user-flow QA.
- Use `$automatic-tester` for repeatable test coverage and runtime smoke gates.
