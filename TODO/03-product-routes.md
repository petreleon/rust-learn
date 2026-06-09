# Product Routes

## App Shell And Routing

- [x] Create a route-based app shell with persistent navigation, account menu,
      workspace switcher, notifications, and contextual actions.
- [ ] Add public routes for landing, login, registration, email verification,
      password recovery, and course discovery.
- [ ] Add learner routes for dashboard, enrolled courses, course detail,
      lesson/content viewing, assessments, rewards, wallet, and settings.
- [ ] Add teacher routes for teaching dashboard, teacher application, course
      authoring, content upload, enrollment management, assessments, student
      progress, and reward candidate review.
- [ ] Add organization routes for organization dashboard, members, courses,
      teacher nominations, reports, reward budget, wallets, and scoped
      permissions.
- [ ] Add platform admin routes for users, roles, permissions, teacher
      application review, reward amount review, fraud blocks, wallet
      reconciliation, exports, system health, and audit logs.
- [x] Move internal workflow-testing controls to `/ops` and clearly separate
      them from the product navigation.

## Authentication And Session UX

- [x] Replace the manual session panel with normal login, logout, registration,
      and account recovery screens.
- [x] Add current-user loading that resolves profile, platform permissions,
      organization memberships, course enrollments, and delegated permissions.
- [x] Persist session state intentionally and document the chosen storage model.
- [x] Handle expired, missing, invalid, and server-revoked sessions with clear
      redirects and messages.
- [x] Show permission-denied states in context instead of only disabling
      controls.
- [x] Add account settings for profile data, email verification status, wallet
      connection status, and notification preferences.

## Learner Experience

- [ ] Build a learner dashboard showing enrolled courses, progress, due work,
      reward status, wallet summary, and recent notifications.
  - [x] Add learner dashboard onward navigation to courses, rewards, and wallet
        product routes.
- [x] Build course discovery with search, filters, organization/course
      metadata, reward availability, and enrollment calls to action.
  - [x] Add a `/courses` product route that shows current course access and
        clearly defers full discovery until catalog data exists.
- [x] `/courses` now consumes `GET /api/courses/catalog`, renders title
      search, enrollment filters, reward filter, organization/teacher/content
      metadata, reward availability, empty states, detail links, and
      request-join actions.
- [x] Build course detail pages with syllabus, instructor information,
      prerequisites, reward policy summary, enrollment status, and access
      requirements.
  - [x] `/courses/[courseId]` consumes
        `GET /api/courses/catalog/{courseId}` and handles signed out, loading,
        success, request join, and course not found. Authored prerequisites stay
        open until the backend schema stores them.
- [x] Build a content/lesson viewer for course material, media processing
      states, completion tracking, and next-step navigation.
  - [x] `/courses/[courseId]/learn` consumes the learner course-learning
        contract, renders text lessons inline, shows safe document/video
        not-rendered or processing copy, supports outline and next/previous
        navigation, and separates signed-out, denied, and failed-processing
        states.
  - [ ] Persisted completion tracking and inline document/video streaming
        remain open until the progress and media playback contracts exist.
- [ ] Build assessment-taking screens when assessment endpoints are ready,
      including attempt state, results, and retry rules.
- [x] Build learner reward history with candidate status, approved amount,
      wallet credit, token transaction, reconciliation status, and helpful
      explanations.
- [ ] Build wallet linking/deposit/retirement screens that make MetaMask and
      platform-wallet requirements understandable.
  - [x] Add wallet summary and self-linking route for linked/unlinked wallet
        states. Deposits, retirements, and audit/history remain open.

## Teacher Experience

- [ ] Build a teacher application journey with draft state, portfolio links,
      scope selection, sponsor context, submission confirmation, and review
      status.
- [ ] Build a teacher dashboard for owned courses, pending student work,
      enrollment requests, reward candidates, and course health.
- [ ] Build course creation and editing flows for title, description, content,
      settings, publishing state, organization ownership, and reward policy.
- [ ] Build content upload UI with progress, validation, processing status,
      retry, and failure explanation.
- [ ] Build enrollment and roster management screens for requests, approvals,
      removals, and waitlist states.
- [ ] Build reward candidate submission and course-scoped teacher approval
      flows without exposing unnecessary platform amount controls.
- [ ] Build student progress views that connect completion evidence to reward
      candidate decisions.

## Organization Experience

- [ ] Build an organization dashboard with member counts, course activity,
      teacher applications, reward volume, wallet balance, and alerts.
- [ ] Build member management for invites, roles, scoped permissions, and
      removal flows.
- [ ] Build organization course management for sponsored courses, publishing
      status, enrollment trends, and reward policy visibility.
- [ ] Build teacher nomination and sponsored-application tracking.
- [ ] Build organization reward reports with date filters, CSV exports,
      approved amounts, payouts, failures, and reconciliation indicators.
- [ ] Build organization wallet and budget views with transaction history and
      permission-aware actions.

## Platform Admin Experience

- [ ] Build a platform admin dashboard for pending teacher applications,
      pending reward amount reviews, payout failures, fraud blocks,
      reconciliation mismatches, and system health.
- [ ] Build teacher application review with queue filters, detail review,
      applicant history, sponsor context, decision reasons, and audit trail.
- [ ] Build reward amount review that is clearly separate from course teacher
      approval.
- [ ] Build fraud-block management for teachers, organizations, courses, and
      reward policies, including audit and revoke flows.
- [ ] Build delegated-permission management with grant, scope, expiration,
      revocation, and usage history.
- [ ] Build exports and reports with clear status, filters, download state, and
      permission-aware availability.
- [ ] Build wallet reconciliation and transaction audit views for platform
      operators.
