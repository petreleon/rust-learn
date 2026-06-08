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

- [ ] Course discovery endpoint with search, filters, lifecycle status,
      organization, teacher, reward summary, and enrollment state.
  - [x] Current frontend refuses to fake discovery: `/courses` shows current
        session course access and a catalog-readiness state until this contract
        exists.
- [ ] Course detail endpoint with syllabus, chapters, content summary, teacher,
      organization, prerequisites, reward policy, and access requirements.
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
- [ ] `/courses` course discovery with search, filters, sort, pagination, and
      empty state.
  - [x] `/courses` product route exists with current course access, useful
        empty state, and honest catalog-readiness state.
- [ ] `/courses/[courseId]` course detail with enrollment action and reward
      policy explanation.
- [ ] `/courses/[courseId]/learn` lesson/content viewer with next-step
      navigation.
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
- [ ] Filter by enrollment status, reward availability, organization, and
      lifecycle visibility.
- [ ] Explain unavailable enrollment: missing email verification, course not
      published, permission denied, suspended course, or request pending.
- [ ] Show reward policy without implying guaranteed payout.
- [ ] Show organization and teacher context without exposing admin-only data.
- [ ] Handle courses with no content, processing content, archived status, or
      unavailable reward policy.

## Lesson And Content Viewer

- [ ] Render content type states: text, document, video, unprocessed upload,
      processing, failed processing, and unavailable object.
- [ ] Preserve progress when navigation changes or the session refreshes.
- [ ] Handle content access denied separately from missing content.
- [ ] Show next lesson and course outline without layout shift.
- [ ] Defer assessment UI until assessment APIs exist, but keep route structure
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
- [ ] Slow or failed media processing does not trap the learner.

## Acceptance Evidence

- [ ] Desktop and mobile screenshots for dashboard, course discovery, course
      detail, reward history, and wallet summary.
  - [x] Browser/Playwright screenshots captured for `/courses`, `/rewards`,
        `/wallet` linked/unlinked states, and mobile menu during this
        checkpoint.
- [ ] Tests for empty learner, enrolled learner, unverified learner, denied
      course access, and reward status transitions.
  - [x] API-helper tests cover learner reward-history success/filtering, wallet
        unlinked `404`, wallet link success, and wallet permission text error.
- [ ] Docker Compose E2E path: login/register, open learner dashboard, open
      course detail, inspect rewards, inspect wallet, scan recent logs.
- [ ] Kubernetes smoke path loads `/learn`, `/courses`, and `/rewards` product
      routes with no console errors or horizontal overflow.

## Current Checkpoint Traceability

Route: `/courses`, `/rewards`, `/wallet`
Persona: learner
Primary job: understand current course access, inspect reward status, and
prepare a wallet for credits.
Backend contracts: `GET /api/me`, `GET /api/reward-candidates/me/history`,
`GET /api/wallets/me`, `POST /api/wallets/me/link`.
Permission and scope rules: resolved session and course effective permissions;
wallet self-access is allowed by backend; reward history only returns permitted
course reward rows.
Shared components: `ProductShell`, learner route bundle, session helper.
Data helper: `web/src/lib/learner.ts`.
States: signed out, loading, success, empty, unlinked wallet, link success,
text/JSON error, timeout/network, session expired.
Edge cases: missing catalog contract, unlinked wallet, wallet link retry,
token transaction without wallet credit, reconciliation-needed rewards.
Rendered proof: in-app Browser signed-out smoke; standalone Playwright
mocked authenticated desktop/mobile checks because Browser lacks interception.
API-helper proof: `npm run test:api-helpers`.
Compose proof: deferred until this learner slice is deployed into Compose.
Kubernetes proof: deferred until this learner slice is deployed into the
Kubernetes web image.
Docs affected: `web/README.md`, learner TODO.
