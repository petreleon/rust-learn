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
- [ ] Course detail endpoint with syllabus, chapters, content summary, teacher,
      organization, prerequisites, reward policy, and access requirements.
- [ ] Learner dashboard endpoint with enrollments, progress, due work,
      notifications, reward summary, and wallet summary.
- [ ] Course progress and content-completion endpoints.
- [ ] Reward history endpoint with status labels, amounts, wallet credit,
      token transaction, reconciliation, and failure reasons.
- [ ] Wallet summary endpoint that clearly distinguishes linked, unlinked,
      pending deposit, confirmed deposit, retirement, and insufficient funds.

## Routes And Screens

- [ ] `/learn` learner dashboard.
- [ ] `/courses` course discovery with search, filters, sort, pagination, and
      empty state.
- [ ] `/courses/[courseId]` course detail with enrollment action and reward
      policy explanation.
- [ ] `/courses/[courseId]/learn` lesson/content viewer with next-step
      navigation.
- [ ] `/rewards` learner reward history.
- [ ] `/wallet` learner wallet summary, link action, deposits, retirements, and
      audit/history.
- [ ] `/settings/account` learner profile and notification preferences.

## Learner Dashboard

- [ ] Show enrolled courses with progress, last activity, next lesson, and
      blocked/suspended/archived course states.
- [ ] Show "continue learning" action only when content is available and the
      user has access.
- [ ] Show due or pending work when assessment/content contracts exist.
- [ ] Show reward summary by human status: pending teacher review, approved
      amount pending, token processing, wallet credited, needs help, failed.
- [ ] Show wallet state with plain next step: link wallet, wait for deposit,
      retry, contact support, or view audit.
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
- [ ] Token transaction exists but wallet credit is missing.
- [ ] Reconciliation repairs state while learner is on the page.
- [ ] A learner has rewards from courses they can no longer access.
- [ ] Slow or failed media processing does not trap the learner.

## Acceptance Evidence

- [ ] Desktop and mobile screenshots for dashboard, course discovery, course
      detail, reward history, and wallet summary.
- [ ] Tests for empty learner, enrolled learner, unverified learner, denied
      course access, and reward status transitions.
- [ ] Docker Compose E2E path: login/register, open learner dashboard, open
      course detail, inspect rewards, inspect wallet, scan recent logs.
- [ ] Kubernetes smoke path loads `/learn`, `/courses`, and `/rewards` product
      routes with no console errors or horizontal overflow.

