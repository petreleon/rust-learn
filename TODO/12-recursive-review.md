# Recursive Roadmap Review

Use this file whenever the TODO tree is reread, expanded, or used to start a
new frontend checkpoint. The point is not to admire the roadmap; the point is to
find where it can still mislead implementation.

## Recursive Review Order

Run these passes from broad to narrow, then loop until no new concrete TODO item
appears.

- [ ] Tree pass: read `TODO/README.md`, then every numbered file in order.
      Check that the folder still describes one coherent frontend strategy.
- [ ] Dependency pass: for each milestone, verify that required backend
      contracts, shared components, API helpers, permissions, and tests are
      named before route work is marked complete.
- [ ] Persona pass: reread each route from the view of the person using it.
      If the route exists mainly for developers, move it to `/ops`.
- [ ] Scope pass: reread every action and ask which platform, organization,
      course, or delegated permission actually authorizes it.
- [ ] State pass: reread every queue, table, form, upload, export, and action
      against loading, empty, denied, stale, conflict, failure, retry, and
      post-action audit states.
- [ ] Runtime pass: reread every milestone and ask what will prove it in a
      browser, Docker Compose, and Kubernetes.
- [ ] Completion pass: look for checkboxes that can be marked done without
      evidence. Rewrite them with acceptance proof.
- [ ] Contradiction pass: compare the TODO files against `README.md`,
      `VISION.md`, `REWARD_LIFECYCLE.md`, `PERMISSIONS.md`, API routes, models,
      and tests. Add contract tasks when the frontend plan outruns the backend.

## Stop Condition

Stop a recursive rewrite only when all of these are true.

- [ ] Every new concern has become a concrete TODO item or a consciously
      deferred note.
- [ ] Each milestone has route, contract, component, state, permission, test,
      Docker Compose, and Kubernetes proof requirements.
- [ ] No item can be completed solely by creating a pretty static screen.
- [ ] No item asks normal users to paste JWTs, know raw internal ids, or toggle
      permissions manually.
- [ ] The operations console is still preserved as `/ops` until equivalent
      product flows are verified.
- [ ] `git diff --check` passes after the rewrite.

## Traceability Rule

Before building a product route, add or fill a small traceability block in the
milestone file or implementation notes.

```text
Route:
Persona:
Primary job:
Backend contracts:
Permission and scope rules:
Shared components:
Data helper:
States:
Edge cases:
Rendered proof:
API-helper proof:
Compose proof:
Kubernetes proof:
Docs affected:
```

Do not implement the route until the traceability block names real contracts and
proof. If the contract does not exist, write or link the contract task first.

## False Agreement Traps

These are places where the roadmap can sound complete while still being weak.

- [ ] "Persona-aware" can drift into role-based authorization. Recheck the
      exact permission and scope.
- [ ] "Dashboard" can drift into decorative metrics. Recheck the decisions the
      dashboard helps the user make.
- [ ] "Empty state" can drift into filler copy. Recheck whether it gives the
      next valid action.
- [ ] "Denied state" can drift into hiding controls. Recheck whether the user
      understands why the action is unavailable.
- [ ] "Backend contract gap" can become a dumping ground. Recheck whether the
      exact endpoint, payload, and error states are named.
- [ ] "Kubernetes smoke" can drift into `/healthz`. Recheck that the product
      route rendered and called the API correctly.
- [ ] "Mobile checked" can mean only page load. Recheck navigation, primary
      action, form controls, tables, and overflow.
- [ ] "Console preserved" can turn into console remaining the main app. Recheck
      the default route and product navigation.

## Findings From The Current Recursive Pass

These issues were found while rereading the current TODO tree and are now
represented in this folder.

- [x] The roadmap needed an explicit recursive review process.
- [x] The roadmap needed route-to-contract-to-test traceability.
- [x] Milestone 6 needed its own plan instead of a one-line summary.
- [ ] Backend contract tasks still need endpoint-level request/response
      sketches once implementation begins.
- [ ] Each route implementation should add traceability notes before coding.
- [ ] The first real implementation checkpoint should start with Milestone 1,
      not with more operations-console polish.
- [x] Learner reward history and wallet linking can be productized with current
      backend contracts.
- [x] Full course discovery, course detail, lesson progress, wallet deposits,
      retirements, and wallet audit/history still cannot be marked complete
      without additional contracts.
- [x] Browser-only QA is not enough for authenticated learner states because
      the current in-app Browser path does not expose network interception or
      storage setup; mocked Playwright QA is required for this checkpoint.
- [x] Teacher content authoring needed a real route rather than another
      operations-console form; `/teach/courses/[courseId]/content` now creates
      chapters and text/article content from structured, permission-aware UI.
- [x] Content authoring could have overclaimed upload/media support; upload
      URL expiry, progress, retry, and media processing recovery remain open
      TODOs until their route and runtime evidence exist.
- [x] Rereading the authoring paths exposed a backend ownership edge case:
      content handlers now reject mismatched course/chapter/content path
      combinations, with regression coverage.
- [x] Teacher enrollment management needed a learner-context contract before a
      real UI could replace raw request/user id forms; the enrollment
      workspace now exposes scoped request and roster summaries.
- [x] Enrollment UI could have overclaimed student progress; the route now
      shows explicit unsupported progress and reward-eligibility cues while
      leaving `/teach/courses/[courseId]/students` open.
- [x] Student progress could have become fake completion math; the students
      route now exposes enrolled learners, content totals, and reward evidence
      while explicitly marking persisted lesson progress unsupported.
- [x] Teacher reward review could have leaked raw candidate/student ids or
      blurred teacher evidence decisions with platform amount review; the
      rewards route now maps visible learners to names, falls back without raw
      ids when learner context is unavailable, submits only teacher
      approval/rejection, and shows stale `409` refresh copy.
- [x] Organization course work could have overclaimed full management when only
      a visibility contract was ready; `/organizations/[organizationId]/courses`
      now delivers the real scoped course list, filters, summaries, permission
      chips, and denied/empty/error/mobile evidence while leaving editing,
      publishing, ownership changes, and reward-policy authoring open.
- [x] Organization member work could have repeated the role-name-only mistake;
      `/organizations/[organizationId]/members` now shows direct, delegated,
      and effective permission summaries plus view-only operator states while
      leaving invite, role mutation, removal, and audit history open.
