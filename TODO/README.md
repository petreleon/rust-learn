# RustLearn TODO

This roadmap is focused on the real product experience that learners, teachers,
organization operators, and platform administrators will use, plus the backend
architecture work needed to keep that product maintainable. The existing
operations console is useful for exercising backend workflows, but it is not the
finished RustLearn user experience.

Recent Backend/API, Docker Compose, Kubernetes, permission, and runtime
verification work has established a healthier baseline. The next priority is to
turn that backend into a route-based, persona-aware, permission-driven web
application.

## Roadmap Files

- [00-self-criticism.md](00-self-criticism.md) - repeated critique protocol for
  deciding whether frontend work is implementation-ready.
- [01-product-rules.md](01-product-rules.md) - product frontend rules,
  baseline, self-critique loop, and edge-case invariants.
- [02-backend-contracts.md](02-backend-contracts.md) - backend/frontend
  contract gaps to resolve before product screens pretend to be complete.
- [03-product-routes.md](03-product-routes.md) - app shell, authentication,
  learner, teacher, organization, and platform admin routes.
- [04-ux-data-layer.md](04-ux-data-layer.md) - design system, reusable
  components, accessibility, and typed frontend data-layer work.
- [05-testing-migration.md](05-testing-migration.md) - frontend testing,
  Docker Compose/Kubernetes verification, migration plan, and milestone gates.
- [06-milestone-01-app-shell-auth.md](06-milestone-01-app-shell-auth.md) -
  detailed app shell, auth, session, and route-guard plan.
- [07-milestone-02-learner.md](07-milestone-02-learner.md) - detailed learner
  dashboard, discovery, course, reward, and wallet plan.
- [08-milestone-03-teacher.md](08-milestone-03-teacher.md) - detailed teacher
  application, course authoring, upload, roster, and reward-review plan.
- [09-milestone-04-organization.md](09-milestone-04-organization.md) -
  detailed organization dashboard, member, course, report, and wallet plan.
- [10-milestone-05-platform-admin.md](10-milestone-05-platform-admin.md) -
  detailed platform admin, review, fraud, delegation, export, and
  reconciliation plan.
- [11-qa-matrix.md](11-qa-matrix.md) - cross-cutting QA matrix for rendered
  frontend, API-helper, Compose, and Kubernetes evidence.
- [12-recursive-review.md](12-recursive-review.md) - recursive reread process,
  traceability rule, false-agreement traps, and current recursive findings.
- [13-milestone-06-ops-deprecation.md](13-milestone-06-ops-deprecation.md) -
  detailed plan for moving the operations console to `/ops`.
- [14-remaining-backend-gaps.md](14-remaining-backend-gaps.md) -
  consolidated list of genuinely buildable backend contracts that are
  not yet implemented (assessments, notifications, pagination, invites,
  etc.).
- [15-testing-gaps.md](15-testing-gaps.md) - remaining page, helper,
  browser, and runtime coverage gaps after the product-route migration.
- [16-platform-walkthrough-after-refactor.md](16-platform-walkthrough-after-refactor.md) -
  post-refactor platform walkthrough notes and route findings.
- [17-observed-missing-product-work.md](17-observed-missing-product-work.md) -
  current objective file for missing product features, ugly interface states,
  recursive verification, and build-ready acceptance checks.
- [18-architecture-modularity-boundaries.md](18-architecture-modularity-boundaries.md) -
  backend modular-monolith refactor plan for Actix/Rust boundaries,
  authorization centralization, DTO separation, and context-by-context
  migration.

## Current Frontend Baseline

- [x] Next.js app exists in `web/`.
- [x] Browser requests can stay on `/api` through the Next proxy.
- [x] `GET /healthz` exists for container and Kubernetes probes.
- [x] Operations console can exercise teacher applications, rewards, reports,
      fraud blocks, delegations, exports, and health checks.
- [x] The operations console is isolated at `/ops`; `/` now resolves the
      product session before sending users to login or session surfaces.
- [x] Real app shell, route structure, and persona-specific navigation exist.
- [x] Learner, teacher, organization, and platform admin experiences are usable
      without treating the UI as an API test panel.

## First Frontend Milestones

- [x] [Milestone 1](06-milestone-01-app-shell-auth.md): App shell, auth screens,
      current-user context, route guards, and permission-aware navigation.
- [x] [Milestone 2](07-milestone-02-learner.md): Learner dashboard, course
      discovery, course detail, reward history, and wallet summary.
- [x] [Milestone 3](08-milestone-03-teacher.md): Teacher application, teacher
      dashboard, course authoring outline, enrollment queue, and reward
      candidate review.
- [x] [Milestone 4](09-milestone-04-organization.md): Organization dashboard,
      member management, reports, wallet budget, and teacher nomination.
- [x] [Milestone 5](10-milestone-05-platform-admin.md): Platform admin
      dashboard, teacher review, reward amount review, fraud blocks,
      delegations, exports, and reconciliation.
- [x] [Milestone 6](13-milestone-06-ops-deprecation.md): Move operations
      console to `/ops`, add product-route Docker Compose/Kubernetes smoke
      checks, and update documentation.

## How To Use This Folder

- Start with [00-self-criticism.md](00-self-criticism.md) before turning any
  item into code.
- Use [12-recursive-review.md](12-recursive-review.md) when rereading or
  expanding this folder.
- Work from the lowest unfinished milestone unless an urgent bug or backend
  contract blocks it.
- Update [02-backend-contracts.md](02-backend-contracts.md) whenever a frontend
  screen needs an endpoint or response shape that does not exist yet.
- Update [11-qa-matrix.md](11-qa-matrix.md) when a new route, state, or runtime
  proof becomes mandatory.
- Use [17-observed-missing-product-work.md](17-observed-missing-product-work.md)
  as the active build backlog for observed missing features until its checks
  are proven by code, tests, and rendered route evidence.
- Use [18-architecture-modularity-boundaries.md](18-architecture-modularity-boundaries.md)
  before large backend refactors that touch API, services, repositories,
  permissions, or infrastructure adapters.
- Keep completed checks honest: mark an item done only when current code and
  runtime evidence prove it.
