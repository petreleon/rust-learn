# TODO 19: Web Architecture Modularity And Firm Boundaries

Last compacted: 2026-06-16.
Planning base: `69e8d309` (`Record Level 2 completion audit`).

Objective: make `web/` a Level 2 frontend modular monolith. Keep Next route
entrypoints thin, move orchestration into feature-owned modules, centralize
API/session/storage boundaries, and keep UI presentational unless a component
owns a small local interaction.

Do not copy backend rings into the frontend. `domain/application/infra/http`
fits Actix; `web/` needs route, feature, API, session, and UI boundaries.

## Why API Exists

`api` means the client adapter to the RustLearn HTTP contract, not another
backend.

- `shared/api`: base URL, `fetch`, timeouts, JSON/text/blob parsing, aborts,
  CSV/blob downloads, and transport errors.
- `feature/api`: context-specific calls such as load teacher enrollments,
  fetch organization members, or decide reward amount.

This keeps URLs, headers, tokens, backend DTO quirks, error envelopes, and
response parsing out of React views. Backend endpoint changes should usually
touch the feature API adapter first, not every screen.

## Boundaries

Flow: `app route -> feature route/controller -> feature view -> shared UI`.

- `app`: Next layouts, pages, params, metadata, and route composition only.
- `features`: `auth`, `session`, `learner`, `teacher`, `organization`,
  `admin`, `wallet`, `kyc`, `ops`.
- `feature/routes`: loading/error/action state, permission gates, refresh,
  redirects, mutations.
- `feature/views`: composed screens with typed props and callbacks.
- `feature/components`: context-specific presentational pieces.
- `feature/model`: labels, selectors, derived state, mappers. No JSX/storage.
- `feature/api`: typed backend calls. No React/CSS/UI imports.
- `shared/session`: token storage, current session, sign-out, permissions,
  visibility refresh.
- `shared/ui`: reusable presentational components. No feature imports.
- `shared/*`: only for rules used by at least two features.

Forbidden:

- `page.tsx` files owning workflow logic or full screens.
- Views calling `fetch`, reading storage/tokens, or parsing backend errors.
- Shared UI importing feature internals.
- Sibling features importing private internals.
- Broad route-code barrel imports without bundle review.
- Fake frontend-only data that hides backend contract gaps.
- Giant one-line TSX used to satisfy the 180-line rule.

## Current Evidence

Good:

- `web/src/app` already owns route entrypoints.
- `web/src/lib` has API helpers for admin/auth/learner/organization/session/
  teacher.
- `web/src/components/*-routes` groups persona routes.
- Primary commands exist: `make web-dev`, `make web-lint`,
  `make web-api-helper-tests`, `make web-build`, `make web-lint-compose`,
  `make web-build-compose`.

Smells:

- Route files mix token reads, session loading, API calls, permissions,
  mutations, filters, error normalization, and JSX.
- Dense files still include deeper teacher course route views,
  account/session pages, and `/ops/page-parts/Home.tsx`.
- Token reads still appear in many route/action modules.
- Route controller hooks are inconsistent across contexts.
- `src/lib/*.ts` barrels can hide large client import surfaces.
- CSS module splits follow file size more than ownership.
- Architecture scans are not repeatable gates yet.

Progress:

- [x] Added `web/scripts/architecture-scan.mjs` plus
  `npm run architecture:scan`, `npm run architecture:scan:strict`, and
  `make web-architecture-scan`.
- [x] Current architecture scan reports no oversized web source files, no
  API modules importing UI/React surfaces, and no view side-effect leaks.
- [x] Current architecture scan reports 51 dense-line findings after the first
  pilot, down from 53 before the pilot.
- [x] Migrated teacher course enrollments into the first workflow slice:
  `features/teacher/course-enrollments/{api,model,route,view}`.
- [x] Kept the old `components/teacher-routes/TeacherCourseEnrollmentsRoute`
  path as a compatibility export.
- [x] Added route tests for signed-out state, feature API loading, decision
  submission, and sign-out behavior.
- [x] Added initial shared API/session/route-state boundaries:
  `shared/api/RequestError`, `shared/session/browserSession`, and
  `shared/route-state/*`.
- [x] Moved the pilot route to shared session/route-state contracts and a
  feature-owned enrollment route model/error adapter.
- [x] Migrated organization members into the second workflow slice:
  `features/organization/members/{api,model,route,view}`.
- [x] Pointed the organization members Next route at the feature route while
  keeping the old `components/organization-routes/OrganizationMembersRoute`
  path as a compatibility export.
- [x] Split organization member directory content into feature-owned
  components while keeping the old content path as a compatibility export.
- [x] Split organization course directory content into feature-owned
  components while keeping the old content path as a compatibility export.
- [x] Split organization reward reports content into feature-owned
  components while keeping the old content path as a compatibility export.
- [x] Split organization wallet audit content into feature-owned components
  while keeping the old content path as a compatibility export.
- [x] Split organization teacher-application tracking content into
  feature-owned components while keeping the old content path as a
  compatibility export.
- [x] Split organization settings panel into feature-owned components while
  keeping the old panel path as a compatibility export.
- [x] Migrated organization settings into
  `features/organization/settings/{api,model,route,view}` with shared
  session/route-state boundaries and a compatibility route export.
- [x] Migrated the admin dashboard into
  `features/admin/dashboard/{api,model,route,view}` with section and CSV
  controller hooks, focused route tests, and a compatibility route export.
- [x] Migrated the admin wallet audit into
  `features/admin/wallets/{api,components,model,route,view}` with wallet
  summary, CSV, and reconciliation panels plus a compatibility route export.
- [x] Migrated delegated-permission administration into
  `features/admin/delegations/{api,components,model,route,view}` with
  create/revoke/list controller hooks and a compatibility route export.
- [x] Migrated fraud-block administration into
  `features/admin/fraud-blocks/{api,components,model,route,view}` with
  list/filter/audit/create/revoke controller hooks, focused route tests, and a
  compatibility route export.
- [x] Migrated reward amount review into
  `features/admin/reward-amount-review/{api,components,model,route,view}` with
  candidate filters, audit, amount-decision controller hooks, focused route
  tests, and a compatibility route export.
- [x] Migrated teacher application review into
  `features/admin/teacher-applications/{api,components,model,route,view}` with
  application filters, audit, decision controller hooks, focused route tests,
  and a compatibility route export.
- [x] Current architecture scan reports 31 dense-line findings after the admin
  teacher application review migration.
- [x] Migrated the top-level teacher dashboard/courses workflow into
  `features/teacher/teaching-workspace/{api,components,model,route,view}`.
- [x] Pointed `/teach` and `/teach/courses` at the feature route, kept
  `components/teacher-routes/TeacherRoute` as a compatibility export, and
  deleted the old dashboard/courses cluster from `components/teacher-routes`.
- [x] Current architecture scan reports 30 dense-line findings after the
  teacher workspace migration, with no long files and no API/view boundary
  violations.
- [x] Migrated the teacher course workspace route into
  `features/teacher/course-workspace/{api,components,model,route,view}` with
  workspace loading, polling, route shell states, focused route tests, and a
  compatibility route export.
- [x] Current architecture scan reports 26 dense-line findings after the
  teacher course workspace migration, with no long files and no API/view
  boundary violations.

Latest pilot proof:

- `make web-architecture-scan` passes in reporting mode with 26 dense-line
  findings, no long files, and no API/view boundary violations.
- `make web-lint` passes with existing warnings.
- `npm run test -- src/shared/api/__tests__/RequestError.test.ts src/shared/route-state/__tests__/normalizeRouteError.test.ts src/features/teacher/course-enrollments/__tests__/TeacherCourseEnrollmentsRoute.test.tsx`
  passes.
- `npm run test -- src/features/organization/members/__tests__/OrganizationMembersRoute.test.tsx`
  passes.
- `npm run test -- src/features/organization/settings/__tests__/OrganizationSettingsRoute.test.tsx`
  passes.
- `npm run test -- src/features/admin/dashboard/__tests__/AdminDashboardRoute.test.tsx`
  passes.
- `npm run test -- src/features/admin/wallets/__tests__/AdminWalletsRoute.test.tsx`
  passes.
- `npm run test -- src/features/admin/delegations/__tests__/AdminDelegationsRoute.test.tsx`
  passes.
- `npm run test -- src/features/admin/fraud-blocks/__tests__/AdminFraudBlocksRoute.test.tsx`
  passes.
- `npm run test -- src/features/admin/reward-amount-review/__tests__/AdminRewardAmountReviewRoute.test.tsx`
  passes.
- `npm run test -- src/features/admin/teacher-applications/__tests__/AdminTeacherApplicationsRoute.test.tsx`
  passes.
- `npm run test -- src/features/teacher/teaching-workspace/__tests__/TeachingWorkspaceRoute.test.tsx`
  passes.
- `npm run test -- src/features/teacher/course-workspace/__tests__/TeacherCourseWorkspaceRoute.test.tsx`
  passes.
- `npm run test -- src/features/teacher/course-enrollments/__tests__/TeacherCourseEnrollmentsRoute.test.tsx`
  passes.
- `npm run test -- src/features/teacher/course-enrollments/__tests__/TeacherCourseEnrollmentsRoute.test.tsx src/components/__tests__/teacher-rewards-route.test.tsx src/components/__tests__/teacher-content-authoring-actions.test.tsx`
  passes.
- `cd web && npm run test` passes with 40 files and 190 tests.
- `make web-api-helper-tests` passes.
- `make web-build` passes.

## Target Shape

```text
web/src/
  app/                  # thin Next route entrypoints
  features/
    admin/ auth/ kyc/ learner/ ops/
    organization/ session/ teacher/ wallet/
  shared/
    access/ api/ formatting/ session/ storage/ ui/
```

Large features split by workflow, not file type:

```text
features/teacher/course-enrollments/
  api/ components/ model/ route/ view/
```

Create a workflow slice when at least two are true: multiple backend calls;
filters/pagination/upload/decision/mutations; permission gates beyond signed-in;
detail plus list/table/workspace; approaching 120 formatted lines; dedicated
controller/model tests needed.

Migration rule: no big-bang folder move. Extract one route at a time, keep
temporary compatibility exports, and delete old modules only after tests and
route evidence prove equivalence.

## Estimate

Assumptions: one focused engineer, no redesign, no new backend contracts,
behavior-preserving refactors. Current scan: about 31k maintained web source
lines, roughly three dozen route entrypoints, existing API helpers, and several
compressed route-controller files.

Useful first slice: 4-7 engineering days.

- Inventory/scans: 0.5-1 day.
- Shared API/session/route-state design: 1-2 days.
- One pilot workflow with tests: 1.5-3 days.
- Verification/TODO proof: 0.5-1 day.

Complete TODO/19: 20-34 engineering days.

- Inventory/boundaries: 1-2 days.
- Shared API/session/access/route-state: 2-4 days.
- Auth/session/account/verification: 2-3 days.
- Learner: 2-4 days.
- Teacher: 4-6 days.
- Organization: 4-6 days.
- Admin: 5-8 days.
- Ops: 1-2 days.
- Scans/tests/Playwright/final audit: 3-5 days.

Calendar: first slice about 1 focused week; full migration about 4-7 focused
weeks with one engineer or 3-5 calendar weeks with two engineers.

Risks: hidden behavior inside compressed TSX, unstable seeded data for runtime
evidence, backend contract gaps, and premature shared abstractions.

## Plan

1. Inventory.
   - Map every `web/src/app/**/page.tsx` to its owner.
   - Record files over 160 lines and lines over 220 characters.
   - Record direct `fetch`, token/storage reads, and API orchestration.
   - Record route imports from `src/lib/*.ts` barrels.

2. Establish shared boundaries.
   - Define shared API, session, access, and route-state rules.
   - Preserve current storage keys and public behavior.
   - Test shared API errors and session storage compatibility.

3. Pilot one workflow route.
   - Prefer teacher enrollments, admin fraud blocks, or organization members.
   - Extract controller hook, pure view, model helpers, feature API, and tests.

4. Migrate by context.
   - Session/auth, learner, teacher, organization, admin, wallet/KYC, then ops.
   - Keep ops isolated from product routes.

5. Cleanup.
   - Share duplicate error/loading/status/formatting/CSV helpers only after two
     features prove the same abstraction.
   - Replace route-code barrel imports where practical.

6. Verify.
   - Add scans for length, dense lines, side effects, and dependency direction.
   - Run `make web-lint`, `make web-api-helper-tests`, `cd web && npm run test`,
     `make web-build`, and Compose checks where relevant.
   - Run Playwright evidence when rendered flows change.

## Self-Critique

- Do not copy backend folders into `web/`.
- Do not treat the 180-line rule as architecture.
- Do not move everything into `features/` before proving one route.
- Do not over-abstract shared helpers before duplication exists.
- Do not let `features/admin` or `features/teacher` become new junk drawers.
- Do not hide backend gaps with frontend-only fake data.

Final rule: preserve useful existing grouping, add firm boundaries, migrate one
representative workflow first, then repeat by context with proof.

## Completion Proof

- Clean `git status --short --branch` at final pushed base.
- Scan: no oversized maintained web files and no dense route-controller lines.
- Scan: views do not call `fetch`, read storage/tokens, or parse backend errors.
- Scan: API modules do not import React, Next routes, CSS modules, or UI.
- `make web-lint`
- `make web-api-helper-tests`
- `cd web && npm run test`
- `make web-build`
- `make web-lint-compose` and `make web-build-compose` where relevant.
- Browser or Playwright route evidence for migrated rendered workflows.

Deferred: product gaps stay in `TODO/02-backend-contracts.md` or
`TODO/17-observed-missing-product-work.md`. This is not a visual redesign.

## Remaining Work

- [x] Build architecture inventory.
- [x] Establish initial shared API/session/route-state boundaries.
- [ ] Apply shared API/session/route-state boundaries to remaining contexts.
- [x] Migrate one pilot workflow with tests.
- [ ] Repeat by context.
- [ ] Remove transitional exports and duplicate helpers.
- [x] Add architecture scans.
- [x] Fix `make web-api-helper-tests` fixture drift.
- [ ] Pay down remaining dense-line findings until strict scan passes.
- [ ] Run and record final verification proof.
