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
  - [x] `/settings/account` now shows current profile, email/KYC readiness,
        self-wallet linked/unlinked/error state, workspace counts, mobile-first
        next-step actions, and disabled notification defaults without exposing
        raw user ids.
  - [ ] Editable notification preference persistence remains open until
        preference read/save endpoints exist.

## Learner Experience

- [ ] Build a learner dashboard showing enrolled courses, progress, due work,
      reward status, wallet summary, and recent notifications.
  - [x] Add learner dashboard onward navigation to courses, rewards, and wallet
        product routes.
  - [x] `/learn` now renders a real learner dashboard from existing learner
        contracts: enrolled courses, continue-learning action, reward summary,
        wallet readiness, recommendations, signed-out state, and first-run
        empty state.
  - [ ] Persisted progress, due work, notification list, and last-activity data
        remain open until those backend contracts exist.
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
        states.
  - [x] `/wallet` now combines self-wallet state with recent learner reward
        history so learners can see wallet credits, active pending credits,
        token-confirmed rewards waiting for credit, and failed/help-needed
        rows without raw internal wallet or user ids.
  - [ ] Real deposits, retirements, MetaMask handoff, and full wallet
        transaction audit remain open until those contracts exist.

## Teacher Experience

- [x] Build a teacher application journey with draft state, portfolio links,
      scope selection, sponsor context, submission confirmation, and review
      status.
  - [x] `/teach/apply` now consumes `GET /api/teacher-applications/me`,
        submits through `POST /api/teacher-applications`, preserves draft
        state and portfolio links, shows submitted/needs-changes/approved/
        rejected/audit states, handles stale duplicate `409` conflicts, and
        avoids raw organization/course/application id entry.
- [x] Build a teacher dashboard for owned courses, pending student work,
      enrollment requests, reward candidates, and course health.
  - [x] `/teach` consumes the current-user session, teaching course dashboard,
        and teacher-application snapshot to show course health, lifecycle
        state, enrollment pressure, reward review pressure, application status,
        and scoped action permissions without raw ids or platform reward amount
        controls.
  - [x] `/teach/courses` lists owned or permitted teaching courses with title
        search, lifecycle filtering, empty/error states, mobile layout, and
        permission-aware links to the course workspace, enrollment, student,
        and reward-review routes.
  - [x] `/teach/courses/[courseId]` opens a real course workspace with
        lifecycle, ownership, teacher roles, action permissions, structured
        chapters/content, processing state, roster pressure, reward review
        pressure, signed-out state, and permission failure state.
  - [x] `/teach/courses/[courseId]/content` opens real content authoring from
        the course workspace, creates chapters and text/article content through
        structured forms, refreshes the outline after successful saves, and
        disables creation when the user lacks course content permission.
  - [x] `/teach/courses/[courseId]/enrollments` opens real enrollment
        management from course cards and the course workspace, filters join
        requests by status, approves/waitlists/rejects requests with decision
        reasons, shows roster access state, and removes course access with
        two-step confirmation without raw learner id entry.
  - [x] `/teach/courses/[courseId]/students` opens real student progress from
        the course workspace, shows enrolled learners, content totals,
        unsupported lesson-progress state, latest enrollment state, and reward
        evidence/counts without raw learner id entry.
  - [x] `/teach/courses/[courseId]/rewards` opens real reward review from
        course cards and the course workspace, filters course-scoped reward
        candidates by status, maps visible learners to names, hides raw ids
        when learner context is unavailable, submits teacher approval/rejection
        decisions only, and refreshes after stale `409` conflicts.
- [ ] Build course creation and editing flows for title, description, content,
      settings, publishing state, organization ownership, and reward policy.
  - [x] Structured chapter and text/article content creation is covered by
        `/teach/courses/[courseId]/content`; title, description, settings,
        publishing state, organization ownership, reward policy, destructive
        editing, and media upload remain open.
- [ ] Build content upload UI with progress, validation, processing status,
      retry, and failure explanation.
- [x] Build enrollment and roster management screens for requests, approvals,
      removals, and waitlist states.
- [ ] Build reward candidate submission and course-scoped teacher approval
      flows without exposing unnecessary platform amount controls.
  - [x] Course-scoped teacher approval/rejection review is covered by
        `/teach/courses/[courseId]/rewards`. Reward candidate submission
        remains open as a product flow.
- [x] Build student progress views that connect completion evidence to reward
      candidate decisions.
  - [x] Current route connects enrolled learners to reward-candidate evidence
        and clearly labels lesson progress as not persisted yet. Persisted
        content completion remains a backend contract gap.

## Organization Experience

- [ ] Build an organization dashboard with member counts, course activity,
      teacher applications, reward volume, wallet balance, and alerts.
  - [x] `/organizations` now opens a real organization workspace selector from
        the current session with multi-org cards, search, capability filters,
        delegated access indicators, role-only membership visibility, and
        denied/signed-out states.
  - [x] `/organizations/[organizationId]` now opens a session-scope
        organization dashboard shell with permission-derived available
        actions, missing-permission explanations, disabled contract-pending
        action buttons, and stale organization handling. Full member counts,
        course activity, reports, wallet, and alert metrics remain open until
        backend contracts exist.
- [ ] Build member management for invites, roles, scoped permissions, and
      removal flows.
  - [x] `/organizations/[organizationId]/members` now consumes
        `GET /api/organizations/{organizationId}/members` to show a scoped
        member directory with search, role and permission filters, pagination,
        role labels, direct/delegated/effective permission summaries, operator
        action readiness, dashboard navigation, and signed-out/denied/empty/
        backend-failure states. Invite, role-change, removal, and audit-history
        actions remain open management work.
- [ ] Build organization course management for sponsored courses, publishing
      status, enrollment trends, and reward policy visibility.
  - [x] `/organizations/[organizationId]/courses` now consumes
        `GET /api/organizations/{organizationId}/courses` to show
        organization-sponsored course summaries with search, lifecycle and
        reward filters, pagination, teacher labels, content readiness,
        enrollment pressure, reward queue pressure, reward policy visibility,
        permission chips, dashboard navigation, and signed-out/denied/empty/
        backend-failure states. Editing, publishing, organization-course
        ownership changes, and reward-policy authoring remain open management
        work.
- [ ] Build teacher nomination and sponsored-application tracking.
  - [x] `/organizations/[organizationId]/teacher-applications` now consumes
        `GET /api/organizations/{organizationId}/teacher-applications` to show
        sponsored/requested application tracking with search, status filters,
        pagination, applicant context, requested scope labels, portfolio links,
        audit hints, submitted/approved states, dashboard navigation, and
        signed-out/denied/empty/backend-failure states.
  - [ ] Nomination submission remains open until searchable applicant lookup
        prevents raw user id entry.
- [ ] Build organization reward reports with date filters, CSV exports,
      approved amounts, payouts, failures, and reconciliation indicators.
  - [x] `/organizations/[organizationId]/reports` now consumes the existing
        all-time organization reward dashboard and CSV contracts, shows reward
        volume, approved amount, sponsored application summary, report-scoped
        wallet balance, course reward volume, export status, refresh, denied,
        empty, and backend-failure states. Date filters, pagination, payout
        failure drill-down, reconciliation indicators, and wallet audit remain
        open contract work.
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
