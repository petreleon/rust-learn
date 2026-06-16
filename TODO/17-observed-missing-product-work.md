# Observed Missing Product Work

Observed from the running app and the current code on 2026-06-12. This file is
a build-scope guide for visible product gaps that still need real workflows,
contracts, or tests before RustLearn can feel complete.

## Recursive Verification Objective

Objective: keep revising this file until it tells a builder exactly what to
build next, in what order, why it matters, what routes/APIs are involved, and
which checks prove the work is done.

Recursive verification loop:

1. Route proof: every high-priority item must name the affected product routes
   and the browser-visible failure or missing workflow.
2. Backend proof: when backend support already exists, the item must say which
   API/helper exists and what frontend/product surface is missing.
3. Build action: every evidence block must lead to concrete build work, not
   only diagnosis.
4. Acceptance checks: every section must include user-visible checks, API/data
   checks, and at least one test or browser verification check.
5. Priority sanity: the priority list must put crashers, global layout, account
   recovery, and core learning loops before lower-risk polish.
6. Duplication check: overlapping items must either be merged or explicitly
   named as cross-cutting dependencies.
7. Decision check: any item blocked by a product decision must state the
   decision needed before implementation.

Definition of perfected for this file:

- A developer can pick the top item and start implementation without asking
  what the feature is supposed to do.
- A product owner can see the build order and understand the tradeoffs.
- A tester can turn each section's checks into a page/API/browser test plan.
- `/ops` is treated as a temporary evidence source, not the target UX.

## Build Packages

P0 - Stabilize the app shell and broken routes:

- Keep `/ops` admin-gated while deciding whether it is an admin-only API
  console, development-only surface, or fully retired after product-route
  replacement.

P1 - Finish account lifecycle:

- Password reset request and completion are now built end to end for local/mock
  delivery; production email/rate-limit hardening remains.
- Resend email verification is now built for local/mock delivery; production
  email/rate-limit hardening remains.
- Shared cross-tab session persistence is implemented with localStorage token
  storage, legacy sessionStorage migration, and sign-out suppression for stale
  legacy tokens.
- Decide the KYC model: self-serve, platform-reviewed, organization-reviewed,
  or external-provider-backed.

P2 - Build the learning loop:

- Enforce or intentionally expose preview access for `/courses/[id]/learn`.
- Persist learner progress and show it consistently on learner and teacher
  surfaces.
- Add assessment authoring, publishing, learner attempts, scoring, max-attempt
  handling, and reward eligibility handoff.
- Finish content edit/delete/unpublish/upload retry/processing-error flows.

P3 - Build rewards and wallet operations:

- Add reward policy management before expanding reward workflows that depend on
  active policies.
- Build learner deposit and retirement intents with tax/gas/confirmation
  states.
- Add token-tax admin configuration.
- Keep the notification center verified as new notification-producing workflows
  are added.

P4 - Build operator and organization workflows:

- Add platform user search, role assignment, and permission preview.
- Add organization discovery/create/request/invite flows.
- Add course creation, metadata editing, lifecycle transitions, and
  organization attachment/ownership flows.
- Replace any remaining `/ops`-only workflow with a product route or mark it
  explicitly out of scope.

P5 - Make tests match product risk:

- Add page-level tests for all major routes and states.
- Add browser smoke for mobile shell, password reset, admin delegation route,
  learner course access, assessment flow, wallet operations, and notification
  center.
- Keep Compose proof for backend-coupled flows such as wallet values, reward
  eligibility, token tax, and reset-token email output.

Decisions needed before implementation:

- KYC ownership and provider model.
- Course learn route access model: enrollment-gated, public preview, or hybrid.
- `/ops` lifecycle: development-only console, admin-only escape hatch, or full
  deprecation.
- Reward policy ownership: platform-only, organization-scoped, course-scoped,
  or inherited with overrides.

## Password Reset

Current evidence:

- `/forgot-password` now submits to `POST /api/auth/forgot-password`, shows
  loading/error/success states, and returns generic copy so account existence is
  not exposed.
- `/reset-password` now pre-fills `?token=...`, submits to
  `POST /api/auth/reset-password`, enforces the registration password policy,
  and handles missing, invalid, expired, reused, weak-password, and success
  states.
- The backend now has `password_reset_tokens` persistence with hashed tokens,
  one-time use, one-hour expiry, active-token invalidation, and bcrypt password
  replacement inside the reset transaction.
- Local development prints a mock reset email with a browser-facing
  `/reset-password?token=...` link controlled by `WEB_PUBLIC_URL`.
- Recovery pages no longer expose internal implementation copy or `/ops`, and
  desktop/mobile browser checks show no overlap or horizontal overflow.

Needed:

- Add production email-provider delivery, bounce/error handling, and rate
  limiting before exposing reset email as a public internet endpoint.
- Add audit metadata if password-reset request/completion events need to appear
  in admin or security review surfaces.
- Consider moving recovery pages from hero layout to a compact task layout if
  later UX review decides account recovery should be denser.

Checks:

- [x] `/forgot-password` accepts a valid email, shows a loading state, and lands
  on a success state that does not reveal whether the email exists.
- [x] Local/mock email output includes a reset URL or token that can be used on
  `/reset-password`.
- [x] `/reset-password` rejects missing, expired, reused, and malformed tokens.
- [x] `/reset-password` enforces the same password policy as registration.
- [x] A successful reset allows login with the new password and rejects the old
  password.
- [x] Desktop and mobile screenshots show no overlapped hero/form content, no
  native-looking inputs/buttons, and no `/ops` link in the recovery nav.
- [x] Browser console has no relevant warnings/errors on forgot/reset flows.

## Session Persistence

Current evidence:

- RustLearn now intentionally uses shared cross-tab auth state: successful login
  stores the JWT under `rustlearn.session.jwt` in localStorage.
- `readStoredSessionToken` migrates old sessionStorage tokens into localStorage
  once, then removes the legacy copy.
- `clearStoredSessionToken` removes both storage locations and writes
  `rustlearn.session.signedOutAt` so stale legacy sessionStorage values are not
  resurrected after sign-out.
- Session unit tests cover empty storage, roundtrip storage, overwrite,
  clear/sign-out, legacy migration, stale legacy suppression, idempotent clear,
  and stable storage key names.

Needed:

- Keep future auth UX copy aligned with the shared cross-tab model; do not add
  tab-scoped session assumptions unless the model is deliberately changed.
- If RustLearn later moves JWTs into httpOnly cookies, migrate this section and
  the storage helpers/tests together.

Checks:

- [x] Login stores the session token in shared localStorage.
- [x] Product routes read the same token after a new tab or refresh.
- [x] Legacy sessionStorage tokens migrate into localStorage and are removed
  from sessionStorage.
- [x] Sign-out removes current and legacy tokens and prevents stale legacy token
  recovery.
- [x] Unit tests cover the storage key names and edge cases above.

## Course Content And Testing

Current evidence:

- Teacher content authoring exists for chapters, text lessons, and file upload
  records.
- Teacher content authoring now exposes text/article content data to the
  teacher workspace, pre-fills the edit form, supports update/delete actions,
  and handles the plain-text backend response from processing retries.
- Learners can read text lessons and open media/document content when ready.
- Assessment API helpers exist in `web/src/lib/learner`, and backend routes
  exist for listing published assessments, learner-safe questions, submitting
  attempts, and listing attempts.
- `/courses/[id]/learn` now mounts a learner assessment panel through the
  feature-owned learner workspace assessment slice. Learners can load published
  assessments, answer learner-safe question prompts, submit attempts, see
  pass/fail feedback, review recent attempts, and see remaining/max-attempt
  state. Preview-mode course access shows assessments without allowing
  submissions.
- Teacher assessment-authoring UI is still not built.

Needed:

- Add teacher assessment authoring: create/edit/publish assessments, questions,
  correct answers, passing score, max attempts, and preview states.
- Add reward eligibility handoff after passing learner assessments.
- Finish the remaining content lifecycle controls: unpublish lesson, upload
  progress/expiry recovery, processing audit/history, and richer
  processing-error recovery.
- Add tests for assessment helpers, assessment UI, content processing states,
  max-attempt behavior, and reward-trigger handoff.

Checks:

- [ ] A teacher can create, edit, publish, and preview an assessment from a
  course-scoped product route.
- [x] A learner can open an assessment from course learning, answer questions,
  submit, and see pass/fail score feedback.
- [x] Attempt history and remaining-attempt counts match backend data after
  refresh.
- [x] Max-attempt and unpublished-assessment states are blocked with clear
  copy.
- [x] Text/article content edit and delete flows render in teacher authoring,
  prefill persisted lesson data after refresh, and pass component, API, and
  browser checks.
- [ ] Content unpublish, upload progress, processing retry, and
  processing-error recovery states render without falling back to `/ops`.
- [ ] Reward eligibility or reward-candidate creation is verified after a
  passing assessment when the course policy requires assessment completion.
- [ ] Unit, API-helper, component, and browser/Playwright coverage exercise the
  happy path plus denied, stale, failed, and empty states.

## KYC Verification

Current evidence:

- `users.kyc_verified` exists and is returned in session, user, organization,
  and teacher roster summaries.
- Registration always creates users with `kyc_verified: false`.
- RustLearn now uses a platform-reviewed KYC model for the first workflow
  slice: users submit details from account settings, and platform reviewers
  decide submissions with `REVIEW_KYC_SUBMISSIONS`.
- `kyc_submissions` persists status, evidence/provider references, reviewer,
  submitted/reviewed timestamps, and rejection reason.
- `/api/kyc/me` returns/submits the signed-in user's KYC status; `/api/kyc/review`
  lists reviewable submissions and `/api/kyc/review/{id}` approves or rejects.
- Approval updates `users.kyc_verified`; rejection leaves the account unverified
  and returns the rejection reason for resubmission.
- Account settings now shows KYC status, start/resubmit form fields, pending
  review copy, and verified/no-action states instead of only a passive badge.
- `/admin/kyc` now provides the product review surface over `/api/kyc/review`,
  with an exact `REVIEW_KYC_SUBMISSIONS` gate, queue/detail rendering, and
  verify/reject decision actions.
- `kyc_audit_events` now records append-only submitted and review-decision
  events with actor, from/to status, reason, metadata, and timestamp; the admin
  KYC detail panel loads that audit history from `/api/kyc/review/{id}/audit`.
- Account settings now renders explicit KYC readiness copy for not started,
  submitted, under review, verified, rejected, expired, and provider-error
  states.
- User wallet linking and user token deposit/retirement service calls now
  require verified KYC; the learner wallet route disables wallet linking and
  explains that wallet, deposit, retirement, and payout actions stay blocked
  until identity review is verified.

Needed:

- Add explicit expired and provider-error transition producers when provider
  or retention rules exist.
- Broaden browser coverage for the signed-in account KYC form and populated
  admin KYC review queue states.

Checks:

- [x] Account settings exposes a KYC start/resume/status action, not only a
  passive `KYC pending` badge.
- [x] Backend routes persist KYC submissions and approve/reject through a
  platform review permission.
- [x] `users.kyc_verified` changes only after a verified review decision.
- [x] KYC API route smoke covers `/api/kyc/me`, `/api/kyc/review`, and
  `/api/kyc/review/{id}`.
- [x] Unit and API-helper tests cover KYC request validation, next-action
  derivation, status fetch, submission payloads, and error handling.
- [x] KYC status transitions are visible for not started, submitted, under
  review, verified, rejected, expired, and provider/error states.
- [x] Backend audit records identify who submitted/reviewed KYC, when it
  changed, and why it was rejected when applicable.
- [x] Any admin/operator review route is permission-scoped and denies users
  without the exact review permission.
- [x] Wallet or payout actions that require KYC are disabled with a clear
  explanation and become available after verification.
- [x] Tests cover KYC submission, review, rejection, resubmission, permission
  denial, and route rendering states.

## Wallet Deposits And Retirements

Current evidence:

- Backend wallet endpoints exist for `/wallets/me/deposits`,
  `/wallets/me/retirements`, and token-tax configuration.
- The learner wallet UI now exposes deposit and retirement intent forms when a
  wallet is linked and KYC is verified.
- Account settings repeats that deposits and retirements are managed from the
  wallet route when available.
- User deposit and retirement service calls now enforce verified KYC before
  creating wallet intents or ledger entries.
- Wallet audit now includes persisted deposit intents, and the learner wallet
  snapshot loads `/wallets/me/audit` so transfer history survives refresh.
- The learner wallet route shows pending, credited, ambiguous, and failed
  deposit-intent states with retry/recreate guidance for failed or ambiguous
  reconciliation.
- Newly created wallet intents render session-local history rows with tax,
  wallet delta, gas payer, platform receiver when returned by the backend,
  MetaMask requirement, and required wallet action.
- Existing wallet integration proof covers platform-paid deposit crediting,
  retirement, wallet value, and audit rows.
- Frontend wallet transfer route tests cover platform-paid deposits,
  insufficient-balance retirements, and user-paid retirement MetaMask
  messaging; backend application tests cover user-paid and platform-paid
  gas-payer behavior for deposits and retirements.

Needed:

- Add frontend tests and Compose proof for both platform-paid and user-paid
  flows.

Checks:

- [x] A linked learner wallet can create deposit and retirement intents from the
  wallet route.
- [x] UI shows tax amount, gas payer, expected wallet delta, platform address,
  and required external wallet action before confirmation.
- [x] Pending, credited, ambiguous, failed, and retry/recreate states are
  visible in wallet history.
- [x] User-paid and platform-paid flows are both covered, including MetaMask or
  permit-required messaging.
- [x] Retirements fail gracefully when the wallet balance is insufficient.
- [x] Compose proof verifies the wallet value and audit rows after a deposit or
  retirement path.

## Session And Permission Display

Current evidence:

- `/session` now renders human role labels and compact permission counts in
  the profile card. Raw permission keys are behind the collapsed "View
  permission details" disclosure.
- Browser QA on June 12, 2026 verified desktop and 390px mobile `/session`
  views for `admin@example.com`: no visible `APPROVE_*` or `SUPER_ADMIN`
  constants, no horizontal overflow, and no console warnings/errors.
- The page reads like a developer session inspector: "Profile, workspace
  scopes, and delegated permissions resolved from the API" is accurate, but not
  normal user-facing workspace copy.
- `ScopeSummary` component tests cover large permission sets, zero
  permissions, delegated-only access, and hidden raw keys before disclosure
  expansion.

Needed:

- Rewrite session page copy for the user task: "What can I access?" rather than
  "What did the API resolve?"
- Add page-level tests for the full `/session` route loading, error,
  signed-out, and signed-in states.

Checks:

- [x] `/session` no longer shows raw permission constants in the first viewport
  for normal users.
- [x] Permission summaries wrap or collapse cleanly at desktop and mobile
  widths, with no cropped text or horizontal overflow.
- [x] The profile card prioritizes user identity, verification/KYC, role, and
  permission count before any details.
- [x] A details view, if present, shows human labels first and raw keys only
  when explicitly expanded.
- [x] Organizations, courses, and delegated-permission sections have clear
  spacing from the top card grid at desktop and mobile widths.
- [x] Browser screenshot and DOM checks confirm no
  `ACCEPT_ORGANIZATION_JOIN_REQUESTAPPROVE_...` style concatenation remains.
- [x] Component tests cover large permission sets, zero permissions, and
  delegated-only access; Browser checks cover desktop and mobile layout.

## Product Shell, Navigation, And Mobile Layout

Current evidence:

- Earlier browser passes over `/session`, `/learn`, `/courses`, `/admin`, and
  `/settings/account` showed the shared `ProductShell` had horizontal overflow
  at mobile width.
- The shared ProductShell CSS module composition now attaches responsive,
  descendant, disabled, hover, notification, and mobile-menu rules to the
  rendered classes.
- A `390x844` browser pass on `/session` now hides desktop nav/toolbar behind
  the mobile menu, keeps the notification panel inside the viewport, and keeps
  the mobile drawer open while nested notifications are opened.
- Desktop scans also report shell nav overflow on multiple pages, especially
  when `Learn`, `Teach`, `Admin`, workspace selection, disabled notifications,
  and the account menu all appear in the first row.
- Browser checks on June 12, 2026 show `/session`, `/learn`, `/courses`,
  `/admin`, and `/settings/account` have no positive document overflow at
  `1280x720` or `390x844`; `/admin` mobile overflow was fixed by letting the
  admin summary cards use two wrapped columns instead of five cramped columns.
- `web/e2e/route-smoke.spec.ts` now covers login, register, forgot password,
  reset password, verify email, signed-out `/session`, signed-out `/admin`, and
  signed-out `/admin/delegations` for route identity, framework-overlay absence,
  console health, and horizontal overflow.
- Public/auth navigation no longer exposes `/ops`; ProductShell only exposes
  the operations console when the resolved session is a platform admin.
- Rendered ProductShell tests assert normal-user account and mobile menus omit
  the operations console while platform admins receive desktop and mobile
  `/ops` links.
- Rendered ProductShell tests also cover active nav `aria-current`,
  signed-out disabled desktop/mobile workspace and notification controls,
  workspace selector routing, and the mobile menu open state.
- The notification bell now opens a real notification center on desktop and
  mobile. The list is capped to the newest 50 notifications and indexed by
  `(user_id, created_at DESC, id DESC)` so accounts with large histories do not
  time out the shell.

Needed:

- Keep notification center coverage in the browser smoke set as new
  notification-producing workflows are added.

Checks:

- [x] At mobile widths, desktop nav and toolbar are hidden while the mobile menu
  is the only primary navigation control.
- [x] `/session`, `/learn`, `/courses`, `/admin`, and `/settings/account` have
  no horizontal overflow at desktop or mobile widths.
- [x] ProductShell tests cover the CSS module split by rendering active nav,
  signed-out controls, workspace controls, and mobile menu states.
- [x] Public auth pages and account menus no longer show `/ops` to normal users.
- [x] The notification bell opens a real notification menu or route with unread
  count, list, empty, loading, error, mark-read, and clear states.
- [x] Notification helpers are covered by component/page tests and at least one
  browser smoke that marks a notification read.

## Admin Routes And Operations Replacement

Current evidence:

- The platform admin dashboard cards link to teacher review, reward amount
  review, fraud controls, delegations, exports, wallets, and system status.
- Direct navigation and dashboard navigation both reach
  `/admin/teacher-applications`, `/admin/rewards/amount-review`,
  `/admin/wallets`, `/admin/fraud-blocks`, `/admin/exports`, and
  `/admin/system`.
- `/admin/delegations` now loads the running backend's raw delegation array
  without a framework overlay. Browser QA on June 12, 2026 loaded 100 rows with
  no `AdminDelegationsRoute` console errors.
- `fetchDelegations()` normalizes raw arrays and paginated objects, including
  empty raw arrays, and rejects malformed objects with `invalid_response` before
  `AdminDelegationsRoute` can dereference undefined list data.
- Admin delegation tests cover the backend raw-array helper contract plus route
  create, list/select, revoke, and permission-gated states.
- `/admin/fraud-blocks` now renders without the previous `FraudBlockDetail`
  audit-history key warning in browser console.
- `/ops` still presents the old business console with many raw inputs,
  permission checkboxes, disabled actions, and API-result output. The product
  routes cover much of this work now, but `/ops` remains an admin-gated
  fallback.

Needed:

- Decide whether `/ops` should remain an admin-only API console, become
  development-only, or be fully retired after product routes cover all actions.
- Keep any remaining ops-only action tracked as a product-route gap until there
  is no need for users to reach `/ops`.

Checks:

- [x] `/admin/delegations` loads without a framework error overlay when the API
  returns zero rows, a raw array, or a paginated object.
- [x] Delegation helper tests cover the backend raw-array response shape, and
  route tests cover list, create, select, revoke, and permission-denied states.
- [x] `/admin` cards link to teacher applications, reward amount review,
  wallets, fraud blocks, delegations, exports, and system status.
- [x] Browser console has no `AdminDelegationsRoute` runtime error and no
  `FraudBlockDetail` key warning after visiting admin subroutes.
- [x] `/ops` is hidden from public/auth/account navigation unless an explicit
  development/admin gate is enabled.
- [ ] Product routes cover every action still listed in `/ops`, or each
  remaining ops-only action has a tracked TODO item.

## Learner Course Lifecycle And Progress

Current evidence:

- `/learn` shows no enrolled courses, zero progress, and recommended courses
  with generated `LifecycleCourse_*` names.
- `/courses` shows a large catalog count and many generated courses, but the
  first viewport is dominated by test-like titles and `Content pending` cards.
- `/courses/60` and `/courses/45` show no syllabus, no active reward policy,
  and no content items. One course is available and one has a pending join
  request.
- Direct navigation to `/courses/60/learn` and `/courses/45/learn` opens the
  learning route even though the audited user is not enrolled or is waiting for
  review. The page shows "Local only" progress and "Course content has not been
  published yet" rather than a clear enrollment gate or preview mode.
- `/courses/[id]/learn` now makes the hybrid access model explicit: enrolled
  learners get tracked progress, non-enrolled accounts with content-view access
  see a read-only "Preview mode" notice, and accounts without content access
  still receive the backend content permission denial.
- Learner progress routes now require enrolled course state and reject content
  ids from other courses. The course learning route skips progress calls in
  preview mode and saves/restores progress only when the API marks progress as
  supported.
- `/learn` now loads saved progress for enrolled courses and shows saved
  activity in the dashboard summary, next-step card, and enrolled course cards.
- Teacher enrollment rosters now mark progress as tracked, and
  `/teach/courses/[id]/students` reports latest viewed lesson, saved activity
  time, saved lesson count, and completion percentage from `course_progress`.
- `/courses/[id]/learn` now exposes learner assessment taking from published
  assessment data and disables attempts in preview mode.
- `/courses/[id]` now shows an assessment entry point only when published
  assessments exist; enrolled learners see "Open assessments", while
  non-enrolled users with content access see "Preview assessments".
- Teacher assessment authoring and reward eligibility handoff after passing
  assessments are still missing.

Needed:

- Keep the hybrid course learning model consistent in navigation: catalog and
  detail links should distinguish "Start learning" from "Preview content" when
  the learner is not enrolled.
- Replace generated/test-like course titles in seeded/demo data used by the
  running product, or hide noisy seed data from normal catalog views.
- Keep improving progress semantics beyond the current "latest viewed lesson"
  model if RustLearn later distinguishes viewed, completed, and assessed
  content.
- Course catalog now shows an explicit visible range such as "2 of 8 shown"
  and "Showing 1-2 of 8 matches" when total matches exceed rendered courses.
- Learner catalog normal browse now hides generated test-style titles such as
  `LifecycleCourse_*` when they are merely available courses, while explicit
  search and direct learner access can still surface the exact course.
- Add assessment entry points inside learner course detail routes and teacher
  course authoring routes.

Checks:

- [x] A non-enrolled learner is either blocked from `/courses/[id]/learn` with a
  clear enrollment message or shown an intentional preview mode.
- [x] Pending join requests cannot access gated lesson content unless preview
  mode is explicitly allowed.
- [x] Persisted progress updates survive refresh and appear consistently on the
  learner dashboard, course learn route, and teacher student route.
- [x] Catalog pages do not expose generated/test-like names in normal product
  smoke data.
- [x] Large catalogs have pagination, load-more, or a clear visible count that
  matches the rendered set.
- [x] Assessment links appear only when assessments exist and handle not
  started, passed, failed, max-attempts, unpublished/empty, and preview-disabled
  states. A separate long-running in-progress attempt state does not exist yet.

## Teacher Course Workflow Gaps

Current evidence:

- `/teach/courses/[id]/content` can create chapters and content, exposes
  teacher-owned text/article data for edit prefill, updates content records,
  deletes content with a two-click confirmation, and keeps processing retries
  wired to the backend's plain-text response.
- `/teach/courses/[id]/enrollments` now marks persisted progress as available
  from the student route and shows backend reward eligibility using active
  course/organization/platform policies plus per-learner candidate counts.
- `/teach/courses/[id]/students` now shows persisted latest viewed lesson,
  saved activity, saved lesson count, completion percentage, reward eligibility,
  and reward candidate counts from backend rows.
- Course cards and course workspace actions disable or explain reward review
  when the current teacher lacks reward-candidate permission.
- Direct `/teach/courses/[id]/rewards` navigation now loads session and course
  context before reward candidates, so a reward permission denial keeps the
  signed-in shell, workspace selector, breadcrumbs, account menu, and course
  title visible; desktop and mobile checks also verify the denial panel does
  not join the error code into the message or create horizontal overflow.
- The teacher course workflow mixes real actions, honest placeholder copy, and
  permission-gated dead ends, so users can click into a route that looks like a
  product feature but cannot be used from the visible role state.

Needed:

- Finish remaining teacher content lifecycle controls: unpublish, upload
  expiry/recovery, processing-error inspection, and audit history.
- Add assessment-authoring routes alongside content authoring.

Checks:

- [x] Content text/article edit and delete flows have component, backend, and
  browser coverage, including persisted edit prefill and two-click delete.
- [ ] Content create/unpublish/retry flows have success, loading,
  validation, conflict, denied, and backend-error tests.
- [x] Enrollment and student routes show persisted progress from backend data
  after refresh.
- [x] Enrollment and student routes show reward eligibility from backend data
  after refresh.
- [x] Course reward review links are visible only when the user can open the
  reward route, or they render a clear disabled state with the missing
  permission.
- [x] `/teach/courses/[id]/rewards` denial keeps the normal signed-in shell,
  workspace selector, breadcrumbs, account menu, readable error text, and no
  desktop/mobile horizontal overflow.
- [ ] Teacher routes include assessment authoring and preview checks for draft,
  published, unpublished, and no-assessment states.

## Organization Workspace Onboarding

Current evidence:

- `/organizations` shows zero organization workspaces for the audited admin
  account, while `/admin` reports thousands of organizations in the platform.
- The organization route gives an empty state, but no visible path to create an
  organization, request access, search organizations, or open a platform org
  directory.
- The workspace selector only contains the personal workspace in this state,
  so organization-specific routes are not discoverable from normal navigation.
- Backend routes exist for organization creation, members, invitations/adding
  members, dashboard, courses, teacher applications, reports, wallet, and
  settings.
- `/organizations` now loads a platform organization directory for platform
  admins with `VIEW_ORGANIZATION`, supports name/id/URL search, and links
  directly to existing organization dashboards without requiring known ids.
- Signed-in users with no organization workspace now see concrete access
  paths: draft an invite request for an existing organization, draft a new
  organization creation request for a platform admin, or review the session
  identity/permission scopes that must receive access.
- Organization members now expose an access-workflow panel: add-by-email is
  shown only when backend operator permissions include member invites, success
  copy says existing-user access is immediate, and pending learner joins link
  to organization courses where pending join counts already appear.
- Organization route controllers now silently refresh the current session when
  a browser tab becomes visible, so ProductShell workspace options reflect
  organization access grants or revokes without requiring a manual reload.
- Organization route states are covered through shared state components and
  focused settings-route tests for signed-out, success, missing organization,
  permission denied, backend error, and save success.

Needed:

- Define separate flows for platform admins, organization owners/operators, and
  ordinary users who need organization access.
- Add organization discovery/create/request/invite entry points that match those
  roles.
- Let platform admins search or open organization dashboards from product UI
  without needing to know an organization id.
- Add empty states that explain how an organization appears in the workspace
  selector.

Checks:

- [x] A platform admin can search and open existing organizations from product
  navigation.
- [x] A user with no organizations sees a clear create/request/access path,
  not only an empty state.
- [x] Organization owners can invite/add members and see pending invite or join
  states from product routes.
- [x] Organization routes preserve clear permission-denied, not-found, empty,
  loading, and backend-error states.
- [x] Workspace selector options update after organization access is granted or
  revoked.

## Account Verification And Session Persistence

Current evidence:

- `/verify-email` provides token verification and a visible resend-verification
  form. `POST /api/auth/resend-verification` returns neutral account copy,
  rotates active tokens for unverified users, and prints the local mock email.
- Registration success now tells users to check email before signing in. The
  API-log mock verification hint is gated to non-production builds.
- Browser QA on June 12, 2026 created a throwaway account from `/register` and
  confirmed the success state, local-only hint, no stale combined local-log
  copy, no `/ops` nav, and no relevant console errors.
- Login, registration, forgot/reset, and verify public nav no longer expose
  `/ops`.
- The frontend stores JWTs in shared `localStorage`, migrates legacy
  `sessionStorage` tokens once, and writes a signed-out marker so stale tabs
  cannot resurrect old tab-scoped tokens after logout.
- Browser QA on June 12, 2026 proved login on `/login?redirect=/session`, a
  second `/session` tab loading `admin@example.com`, second-tab sign-out, and
  first-tab reload returning to "Sign in required" without console errors.
- Account settings notification preferences now connect to a real shell
  notification center.

Needed:

- Add production email-provider delivery, bounce/error handling, and rate
  limiting for verification emails.
- Add an in-product verification delivery status once production email delivery
  exists.
- Add an explicit production auth lifetime policy such as remember-me,
  inactivity expiry, refresh, or device/session revocation.

Checks:

- [x] A newly registered user can request another verification email without
  exposing whether an address exists.
- [x] `/verify-email` handles valid, missing, malformed, expired, reused, and
  already-verified tokens.
- [x] Registration success copy is user-facing in production and development
  hints are gated to local builds.
- [x] Opening a new tab has an intentional, tested auth behavior with clear
  redirect copy.
- [x] Login/register/forgot/reset/verify pages do not expose `/ops` in normal
  navigation.

## Platform Configuration, Roles, And Policy Management

Current evidence:

- Backend APIs exist for platform users and role assignment:
  `/api/user`, `/api/user/{id}`, `/api/user/{id}/role`, and `/api/roles`.
  There is no matching platform user-management route in `web/src/app` and no
  admin helper for listing users or assigning platform roles.
- Backend APIs exist for course creation, course update, and lifecycle changes:
  `POST /api/courses`, `PUT /api/courses/{id}`, and
  `PUT /api/courses/{id}/lifecycle`. Teacher and organization course routes
  list existing courses, but the product UI does not expose course creation,
  metadata editing, publication, archiving, or lifecycle transition controls.
- Organization course pages explicitly say editing, publishing, and
  organization-course ownership changes remain separate route work.
- Backend APIs exist for reward policy creation/listing at
  `/api/reward-policies`, but there is no product route/helper for reward policy
  creation, policy scope, amount rules, active/inactive state, or policy audit.
  Current learner/teacher/org pages mostly show counts such as "No active
  policy" or "Reward policies".
- Fraud blocks can target a `reward_policy_id`, but the UI gives operators no
  way to search/select a reward policy by name/context before blocking it.
- Backend APIs exist for token-tax configuration:
  `/api/wallets/token-taxes`, `/api/wallets/token-taxes/deposit`, and
  `/api/wallets/token-taxes/retire`. There is no visible admin configuration UI
  for deposit/retirement tax values or effective policy history.
- `/admin/wallets` now includes token-tax configuration for platform admins
  with `SET_DEPOSIT_TAX` or `SET_RETIRE_TAX`; tax-only admins can update
  configured tax values without loading wallet audit data.

Needed:

- Add platform user and role management routes with search, filters, role
  assignment, permission preview, audit history, and permission-denied states.
- Add teacher/organization course creation, course metadata editing, lifecycle
  transition, ownership/organization attachment, and publish/archive controls.
- Add reward policy management for platform, organization, and course scopes:
  create, list, inspect, activate/deactivate, validate coverage, and audit.
- Add reward-policy picker/search to fraud-block creation and reward
  operations wherever a numeric policy id is currently required.
- Add token-tax audit trail, effective history, and clear user-facing impact on
  deposit/retirement flows.

Checks:

- [ ] Platform admins can search users, inspect user roles/permissions, assign
  roles, and see audit feedback without using `/ops`.
- [ ] Teacher or organization operators with the right permissions can create a
  course, edit course metadata, and move lifecycle status through valid
  transitions.
- [ ] Invalid lifecycle transitions, missing permissions, stale updates, and
  archived/deleted course states are blocked with clear messages.
- [ ] Reward policies can be created, listed, activated/deactivated, scoped,
  audited, and selected by name/context instead of numeric id only.
- [ ] Reward candidate creation surfaces the policy that makes a reward event
  eligible, or explains which policy is missing.
- [x] Deposit and retirement tax configuration can be viewed and changed only by
  authorized admins.
- [ ] Token-tax audit rows, effective history, and visible downstream wallet
  impact are surfaced.
- [ ] Browser/page tests cover user-role management, course lifecycle changes,
  reward policy management, policy-backed fraud blocks, and token-tax updates.

## Page-Level Product Tests

Current evidence:

- `TODO/15-testing-gaps.md` still calls out zero page-level integration tests
  for `web/src/app/` pages.
- Component and API-helper tests exist, but product routes can still regress at
  the page boundary: URL params, redirects, storage/session setup, shell
  composition, route-level loading/error states, and browser navigation.
- Playwright route smoke now covers auth pages plus signed-out `/session`,
  `/admin`, and `/admin/delegations`, including mobile overflow checks.
- `make web-page-tests` now runs fast Vitest page/route-level suites through
  `web/scripts/page-tests.mjs` for ProductShell, admin, learner, organization,
  session, and teacher routes without Playwright, screenshots, Docker Compose,
  or persisted browser state.

Needed:

- Add page-level tests for auth, session, learner, teacher, organization,
  admin, wallet, and account routes.
- Include signed-out, loading, success, denied, not-found, backend-error,
  timeout, mobile, and post-action states.
- Keep Playwright smoke tests for real browser confidence, but make page-level
  tests fast enough to run in ordinary frontend CI.

Checks:

- [ ] `web/src/app/` has page-level coverage for auth, session, learner,
  teacher, organization, admin, wallet, and account routes.
- [ ] Tests verify route params, redirects, session storage behavior, and
  ProductShell composition.
- [ ] Tests cover signed-out, loading, success, denied, not-found, backend
  error, timeout, empty, and post-action states.
- [ ] Mobile-width render tests or Playwright screenshots catch overlap,
  clipping, unreadable text, and horizontal overflow.
- [x] The page-level test command is documented and runs without depending on
  the full Docker Compose stack.

## Build Order

1. Stabilize shell, route crashes, and navigation discoverability. This includes
   ProductShell/auth/session CSS composition, mobile overflow,
   `/admin/delegations`, admin dashboard links, and `/ops` gating.
2. Finish account lifecycle. This includes password reset, resend email
   verification, production-safe verification copy, intentional session
   persistence, and the KYC ownership decision.
3. Build the learning loop. This includes course access gating or preview mode,
   persisted progress, assessment authoring/taking/scoring, content lifecycle,
   and teacher reward-route permission clarity.
4. Build rewards and wallet operations. This includes reward policy management,
   deposit and retirement intents, token-tax configuration, wallet history, and
   the notification center.
5. Build operator and organization workflows. This includes platform users and
   roles, organization discovery/create/request/invite, course creation,
   lifecycle transitions, and organization attachment/ownership.
6. Add page-level and browser verification. This should run alongside every
   package, but it becomes a standalone cleanup only after the highest-risk
   routes have stable behavior.
