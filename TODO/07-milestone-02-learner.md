# Milestone 2: Learner Experience

Goal: make RustLearn feel like a learning product, not an admin console. A
learner should be able to discover courses, understand enrollment status, see
progress, inspect rewards, and understand wallet state.

## Self-Criticism Before Building

- [ ] If the learner must know a course id, reject the flow.
- [ ] If the dashboard only shows static cards because progress endpoints are
      missing, mark the contract gap instead of pretending it is complete.
- [ ] If rewards are shown as token jargon without course context and next
      action, rewrite the copy.
- [ ] If empty states do not tell a new learner how to start, reject them.
- [ ] If mobile course discovery requires awkward horizontal scanning, reject
      the layout.

## Backend Contracts

- [x] Course discovery endpoint with search, filters, lifecycle status,
      organization, teacher, reward summary, and enrollment state.
  - [x] Current frontend refuses to fake discovery: `/courses` shows current
        session course access and a catalog-readiness state until this contract
        exists.
- [x] `GET /api/courses/catalog` now returns learner-visible published courses
      plus scoped own courses with title search, organization filter,
      lifecycle filter, enrollment filter, reward filter, organizations,
      teachers, content summary, reward summary, enrollment state, and access
      flags.
- [x] Course detail endpoint with syllabus, chapters, content summary, teacher,
      organization, prerequisites, reward policy, and access requirements.
  - [x] `GET /api/courses/catalog/{courseId}` returns the learner course
        contract and chapter/content summaries. `description`, `topics`, and
        `prerequisites` remain nullable/empty until the course schema supports
        them.
- [x] Learner lesson endpoint with course summary, ordered chapters, ordered
      content items, active first lesson, upload/media display state, and
      scoped content permission errors.
  - [x] `GET /api/courses/catalog/{courseId}/learn` returns text-ready,
        uploaded, processing, failed-processing, unprocessed-upload, and
        unavailable states while keeping persisted progress explicitly
        unsupported.
- [ ] Learner dashboard endpoint with enrollments, progress, due work,
      notifications, reward summary, and wallet summary.
- [ ] Course progress and content-completion endpoints.
- [ ] Reward history endpoint with status labels, amounts, wallet credit,
      token transaction, reconciliation, and failure reasons.
  - [x] Frontend helper and `/rewards` route consume the existing learner
        reward-history contract with status filters, course context, approved
        amount, wallet credit, token transaction, and reconciliation copy.
- [ ] Wallet summary endpoint that clearly distinguishes linked, unlinked,
      pending deposit, confirmed deposit, retirement, and insufficient funds.
  - [x] Frontend helper and `/wallet` route handle the existing linked,
        unlinked, and link-wallet response states without raw ids or pasted
        tokens.

## Routes And Screens

- [ ] `/learn` learner dashboard.
  - [x] `/learn` now routes learners to `/courses`, `/rewards`, and `/wallet`
        from the product workspace.
- [x] `/courses` course discovery with search, filters, sort, pagination, and
      empty state.
  - [x] `/courses` product route exists with current course access, useful
        empty state, and honest catalog-readiness state.
- [x] `/courses` now consumes the learner catalog contract with search,
      enrollment filters, reward-only filter, refresh, empty state, detail
      links, and request-join action.
- [x] `/courses/[courseId]` course detail with enrollment action and reward
      policy explanation.
  - [x] Detail route shows organization, teacher, content summary, syllabus,
        reward policy summary, enrollment state, request-join action, signed
        out state, and course-level not-found/error handling.
- [x] `/courses/[courseId]/learn` lesson/content viewer with next-step
      navigation.
  - [x] The route consumes
        `GET /api/courses/catalog/{courseId}/learn`, renders outline,
        selected lesson, next/previous actions, signed-out state, denied
        state, and media-processing failures without raw ids.
- [x] `/rewards` learner reward history.
- [ ] `/wallet` learner wallet summary, link action, deposits, retirements, and
      audit/history.
  - [x] `/wallet` product route exists with wallet summary, unlinked empty
        state, link action, refresh action, success notice, and action-error
        notice.
- [ ] `/settings/account` learner profile and notification preferences.

## Learner Dashboard

- [ ] Show enrolled courses with progress, last activity, next lesson, and
      blocked/suspended/archived course states.
  - [x] Show current course scopes and lifecycle status from the resolved
        session while progress/last-activity contracts are still missing.
- [ ] Show "continue learning" action only when content is available and the
      user has access.
- [ ] Show due or pending work when assessment/content contracts exist.
- [ ] Show reward summary by human status: pending teacher review, approved
      amount pending, token processing, wallet credited, needs help, failed.
  - [x] `/rewards` shows human status, next step, wallet credit, token
        transaction, and reconciliation/failed copy.
- [ ] Show wallet state with plain next step: link wallet, wait for deposit,
      retry, contact support, or view audit.
  - [x] `/wallet` shows linked/unlinked states, link action, retryable refresh,
        and action-specific success/error notices.
- [ ] Show useful first-run empty state with course discovery call to action.

## Course Discovery And Detail

- [ ] Search by title, organization, teacher, and topic when supported.
  - [x] Title search is implemented. Teacher/topic search stays open because
        those fields are not queryable in the current schema.
- [ ] Filter by enrollment status, reward availability, organization, and
      lifecycle visibility.
  - [x] Backend supports enrollment, reward, organization, and lifecycle
        filters. `/courses` exposes enrollment and reward filters; organization
        picker UI remains open until a learner-safe organization lookup is
        available.
- [ ] Explain unavailable enrollment: missing email verification, course not
      published, permission denied, suspended course, or request pending.
  - [x] Pending, waitlisted, rejected, enrolled, unavailable/not-published, and
        cannot-request states have product copy. Email-verification and
        suspended-course specific explanations remain open.
- [x] Show reward policy without implying guaranteed payout.
- [x] Show organization and teacher context without exposing admin-only data.
- [ ] Handle courses with no content, processing content, archived status, or
      unavailable reward policy.
  - [x] No-content and unavailable reward states render. Processing media
        state and richer archived/suspended treatment remain open for the
        media-processing/status and lifecycle UX contracts.

## Lesson And Content Viewer

- [ ] Render content type states: text, document, video, unprocessed upload,
      processing, failed processing, and unavailable object.
  - [x] Text lessons render inline. Video/media states render ready, uploaded,
        unprocessed upload, processing, failed processing, and unavailable
        product copy.
  - [ ] Inline document preview and video streaming remain open; non-text ready
        content currently shows a safe not-rendered-yet state.
- [ ] Preserve progress when navigation changes or the session refreshes.
  - [x] Local selected lesson state changes with outline and next/previous
        actions. Persisted completion/progress waits for the progress
        endpoint.
- [x] Handle content access denied separately from missing content.
- [x] Show next lesson and course outline without layout shift.
- [x] Defer assessment UI until assessment APIs exist, but keep route structure
      ready.

## Learner Edge Cases

- [ ] Unverified email blocks enrollment or rewards where backend requires it.
- [ ] Course becomes suspended while learner is viewing it.
- [ ] Reward candidate changes status after page load.
- [ ] Wallet is not linked when reward history exists.
  - [x] `/wallet` treats `404 Wallet not linked` as a product empty state, not
        as a broken page.
- [ ] Token transaction exists but wallet credit is missing.
  - [x] `/rewards` separates token transaction visibility from wallet-credit
        visibility.
- [ ] Reconciliation repairs state while learner is on the page.
  - [x] `/rewards` exposes `needs_reconciliation` as a help-needed status.
- [ ] A learner has rewards from courses they can no longer access.
- [x] Slow or failed media processing does not trap the learner.
  - [x] Processing and failed-processing states remain navigable and expose
        staff-facing retry/replace copy without blocking the outline.

## Acceptance Evidence

- [ ] Desktop and mobile screenshots for dashboard, course discovery, course
      detail, reward history, and wallet summary.
  - [x] Browser/Playwright screenshots captured for `/courses`, `/rewards`,
        `/wallet` linked/unlinked states, and mobile menu during this
        checkpoint.
  - [x] Playwright screenshots captured for `/courses` catalog, mobile catalog
        menu, `/courses/[courseId]` detail flow, request-join success, signed
        out state, and course-detail `404` state.
  - [x] Browser/Playwright screenshots captured for
        `/courses/[courseId]/learn` desktop lesson switching, mobile first
        viewport, denied content state, and signed-out state.
- [ ] Tests for empty learner, enrolled learner, unverified learner, denied
      course access, and reward status transitions.
  - [x] API-helper tests cover learner reward-history success/filtering, wallet
        unlinked `404`, wallet link success, and wallet permission text error.
  - [x] Rust API tests cover learner catalog summaries, pending enrollment
        state, published visibility, own draft visibility, and hidden unscoped
        draft detail.
  - [x] API-helper tests cover course catalog filters, course detail, join
        request, and detail `404` normalization.
  - [x] API-helper tests cover learner course-learning success and text `403`
        normalization.
  - [x] Rust API tests cover learner course-learning response shape, active
        content, media state mapping, and unscoped learner `403`.
- [ ] Docker Compose E2E path: login/register, open learner dashboard, open
      course detail, inspect rewards, inspect wallet, scan recent logs.
- [ ] Kubernetes smoke path loads `/learn`, `/courses`, and `/rewards` product
      routes with no console errors or horizontal overflow.

## Current Checkpoint Traceability

Route: `/courses`, `/courses/[courseId]`, `/courses/[courseId]/learn`,
`/rewards`, `/wallet`
Persona: learner
Primary job: discover visible courses, inspect course detail, request
enrollment, open course lessons, inspect reward status, and prepare a wallet
for credits.
Backend contracts: `GET /api/me`, `GET /api/reward-candidates/me/history`,
`GET /api/courses/catalog`, `GET /api/courses/catalog/{courseId}`,
`GET /api/courses/catalog/{courseId}/learn`,
`POST /api/courses/{courseId}/join-requests`, `GET /api/wallets/me`,
`POST /api/wallets/me/link`.
Permission and scope rules: resolved session and course effective permissions;
wallet self-access is allowed by backend; reward history only returns permitted
course reward rows; lesson viewing requires scoped `VIEW_CONTENT`.
Shared components: `ProductShell`, learner route bundle, session helper.
Data helper: `web/src/lib/learner.ts`.
States: signed out, loading, success, empty, catalog filters, pending
enrollment, request-join success, course not found, lesson ready, uploaded
media, processing media, failed processing, unavailable content, content
permission denied, unlinked wallet, link success, text/JSON error,
timeout/network, session expired.
Edge cases: unscoped draft hidden from learner catalog, own draft visible by
course role, course-detail `404` keeps signed-in shell, unlinked wallet, wallet
link retry, token transaction without wallet credit, reconciliation-needed
rewards, learner lesson `403` keeps signed-in shell.
Rendered proof: in-app Browser signed-out smoke; standalone Playwright
mocked authenticated desktop/mobile checks because Browser lacks interception.
API-helper proof: `npm run test:api-helpers`.
Rust proof: `./scripts/run-host-tests.sh cargo test --test course_discovery`.
Compose proof: deferred until this learner slice is deployed into Compose.
Kubernetes proof: deferred until this learner slice is deployed into the
Kubernetes web image.
Docs affected: `web/README.md`, learner TODO.
