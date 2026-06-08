---
name: manual-tester
description: "Manual rendered frontend QA for RustLearn or similar web apps through Browser/Playwright: inspect desktop/mobile UI, exercise user flows, verify copy/states/permissions, capture screenshots, and report findings. Use when asked to manually test, QA, inspect, reproduce, or validate frontend user flows."
---

# Manual Tester

## Core Posture

Test like a real user with imperfect data, not like a developer proving a happy
path. Prefer visible rendered evidence over assumptions from code or build logs.

When working in RustLearn, read `TODO/11-qa-matrix.md` and the relevant
milestone file before validating a route.

## Workflow

1. Define the flow under test as `entry route -> user action -> expected visible
   result`.
2. Open the route in Browser first when available; fall back to Playwright only
   when Browser invocation fails or is unavailable.
3. Check page identity, not blank, no framework overlay, console health,
   screenshot evidence, and interaction proof.
4. Test desktop and one mobile viewport at minimum. For RustLearn mobile, watch
   first viewport clarity and horizontal overflow.
5. Exercise at least one non-happy state: denied, empty, validation error,
   conflict, expired session, network failure, or backend failure.
6. Record findings before summary. Include reproduction steps, observed result,
   expected result, evidence, likely file/owner, and severity.
7. After fixes, rerun the same flow and compare evidence.

## RustLearn Focus Areas

- Product routes should not render the operations console except `/ops`.
- Normal users should not need manual JWTs, raw ids, or permission checkboxes.
- Permission-denied states should explain missing capability without leaking
  sensitive admin details.
- Tables, filters, forms, uploads, exports, and action buttons need mobile
  usability checks.
- Docker Compose and Kubernetes rendered checks should prove product routes, not
  only health endpoints.

## QA Report Shape

Use a concise QA report:

- Summary
- Environment
- Flow Tested
- Findings
- Checks
- Evidence
- Remaining Risk

