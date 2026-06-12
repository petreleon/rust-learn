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

- Fix ProductShell/auth/session CSS module composition so desktop and mobile
  layouts stop overflowing.
- Fix `/admin/delegations` response-shape crash.
- Remove or gate `/ops` from normal auth/account navigation.
- Link `/admin` dashboard cards to existing admin subroutes.
- Add route-level smoke coverage for the shell, auth pages, `/session`,
  `/admin`, and `/admin/delegations`.

P1 - Finish account lifecycle:

- Password reset request and completion are now built end to end for local/mock
  delivery; production email/rate-limit hardening remains.
- Resend email verification is now built for local/mock delivery; production
  email/rate-limit hardening remains.
- Decide and implement intentional session persistence behavior across tabs.
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
- Session persistence model: tab-scoped session storage or cross-tab shared
  auth state.
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

## Course Content And Testing

Current evidence:

- Teacher content authoring exists for chapters, text lessons, and file upload
  records.
- The authoring UI explicitly says processing retry and destructive editing
  controls remain separate until contracts are complete.
- Learners can read text lessons and open media/document content when ready.
- Assessment API helpers exist in `web/src/lib/learner`, and backend routes
  exist for listing published assessments, submitting attempts, and listing
  attempts.
- No route currently imports or renders those assessment helpers. There is no
  learner assessment-taking UI and no teacher assessment-authoring UI.

Needed:

- Add teacher assessment authoring: create/edit/publish assessments, questions,
  correct answers, passing score, max attempts, and preview states.
- Add learner assessment taking: attempt history, remaining attempts, answer
  entry, submit flow, score/pass result, retry rules, and reward eligibility
  handoff.
- Finish content lifecycle controls: edit lesson, delete/unpublish lesson,
  requeue/retry processing, show upload progress, and expose processing errors
  in a recoverable workflow.
- Add tests for assessment helpers, assessment UI, content processing states,
  max-attempt behavior, and reward-trigger handoff.

Checks:

- [ ] A teacher can create, edit, publish, and preview an assessment from a
  course-scoped product route.
- [ ] A learner can open an assessment from course learning, answer questions,
  submit, and see pass/fail score feedback.
- [ ] Attempt history and remaining-attempt counts match backend data after
  refresh.
- [ ] Max-attempt and unpublished-assessment states are blocked with clear
  copy.
- [ ] Content edit, unpublish/delete, upload progress, processing retry, and
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
- The account route only displays `KYC not verified`; it does not provide a
  verification workflow.
- No backend route appears to submit KYC data, approve/reject KYC, attach KYC
  evidence, or update the field through a scoped permission.

Needed:

- Decide whether KYC is self-serve, platform-reviewed, organization-reviewed,
  or external-provider-backed.
- Add KYC submission and review data models, including status, evidence
  references, reviewer, timestamps, rejection reason, and audit events.
- Add user-facing account states for not started, submitted, under review,
  verified, rejected, expired, and provider/error conditions.
- Add scoped admin/operator review routes only if humans in RustLearn approve
  KYC.
- Gate future wallet or payout operations that require KYC with clear reasons
  instead of silently showing `KYC pending`.

Checks:

- [ ] Account settings exposes a KYC start/resume/status action, not only a
  passive `KYC pending` badge.
- [ ] KYC status transitions are visible for not started, submitted, under
  review, verified, rejected, expired, and provider/error states.
- [ ] Backend audit records identify who submitted/reviewed KYC, when it
  changed, and why it was rejected when applicable.
- [ ] Any admin/operator review route is permission-scoped and denies users
  without the exact review permission.
- [ ] Wallet or payout actions that require KYC are disabled with a clear
  explanation and become available after verification.
- [ ] Tests cover KYC submission, review, rejection, resubmission, permission
  denial, and route rendering states.

## Wallet Deposits And Retirements

Current evidence:

- Backend wallet endpoints exist for `/wallets/me/deposits`,
  `/wallets/me/retirements`, and token-tax configuration.
- The learner wallet UI says deposits and retirements are not available in the
  UI yet.
- Account settings repeats that deposits and retirements are managed from the
  wallet route when available.

Needed:

- Add learner wallet actions for deposit and retirement intents.
- Show gas-payer mode, tax amount, expected wallet delta, platform receiver,
  MetaMask/permit requirements, pending confirmation, credited, ambiguous, and
  failed states.
- Add wallet-history rows for deposit/retirement intents alongside reward
  credits.
- Add frontend tests and Compose proof for both platform-paid and user-paid
  flows.

Checks:

- [ ] A linked learner wallet can create deposit and retirement intents from the
  wallet route.
- [ ] UI shows tax amount, gas payer, expected wallet delta, platform address,
  and required external wallet action before confirmation.
- [ ] Pending, credited, ambiguous, failed, and retry/recreate states are
  visible in wallet history.
- [ ] User-paid and platform-paid flows are both covered, including MetaMask or
  permit-required messaging.
- [ ] Retirements fail gracefully when the wallet balance is insufficient.
- [ ] Compose proof verifies the wallet value and audit rows after a deposit or
  retirement path.

## Session And Permission Display

Current evidence:

- `/session` renders raw permission constants in the profile card, for example
  `ACCEPT_ORGANIZATION_JOIN_REQUEST` and `APPROVE_CENTRALIZED_TRANSFER`.
- The permissions run together and overflow/crop horizontally, making the card
  look broken rather than intentionally summarized.
- The visible `+109` count is useful, but it appears after several unreadable
  raw constants instead of being the primary summary.
- The page reads like a developer session inspector: "Profile, workspace
  scopes, and delegated permissions resolved from the API" is accurate, but not
  normal user-facing workspace copy.
- The same CSS Modules split issue appears here: `.permissionList` is composed
  from `web/src/app/session/page.module/01.module.css`, while the flex/wrap
  rules live in `web/src/app/session/page.module/02.module.css`, so the rendered
  permission list misses the wrapping layout and keeps `white-space: nowrap`.
- Organizations, courses, and delegated-permission sections feel too tightly
  attached to the top cards, which suggests other composed section styles may
  also be missing or too weak.

Needed:

- Repair the session CSS module split so `.permissionList`, workspace section
  spacing, and responsive rules apply to the rendered classes.
- Replace raw permission constants with human labels, grouped categories, or a
  compact "View permissions" disclosure.
- Make the profile card show the useful summary first: role, verification, KYC,
  number of effective permissions, and delegated access status.
- Keep raw permission keys available only in an inspector/debug details view,
  copy-to-clipboard panel, or admin-oriented route.
- Rewrite session page copy for the user task: "What can I access?" rather than
  "What did the API resolve?"

Checks:

- [ ] `/session` no longer shows raw permission constants in the first viewport
  for normal users.
- [ ] Permission summaries wrap or collapse cleanly at desktop and mobile
  widths, with no cropped text or horizontal overflow.
- [ ] The profile card prioritizes user identity, verification/KYC, role, and
  permission count before any details.
- [ ] A details view, if present, shows human labels first and raw keys only
  when explicitly expanded.
- [ ] Organizations, courses, and delegated-permission sections have clear
  spacing from the top card grid at desktop and mobile widths.
- [ ] Browser screenshot and DOM checks confirm no
  `ACCEPT_ORGANIZATION_JOIN_REQUESTAPPROVE_...` style concatenation remains.
- [ ] Component/page tests cover large permission sets, zero permissions,
  delegated-only access, and mobile layout.

## Product Shell, Navigation, And Mobile Layout

Current evidence:

- Manual browser passes over `/session`, `/learn`, `/courses`, `/admin`, and
  `/settings/account` show the shared `ProductShell` has horizontal overflow at
  mobile width.
- The shared ProductShell CSS module composition now attaches responsive,
  descendant, disabled, hover, notification, and mobile-menu rules to the
  rendered classes.
- A `390x844` browser pass on `/session` now hides desktop nav/toolbar behind
  the mobile menu, keeps the notification panel inside the viewport, and keeps
  the mobile drawer open while nested notifications are opened.
- Desktop scans also report shell nav overflow on multiple pages, especially
  when `Learn`, `Teach`, `Admin`, workspace selection, disabled notifications,
  and the account menu all appear in the first row.
- Public/auth navigation no longer exposes `/ops`; ProductShell only exposes
  the operations console when the resolved session is a platform admin.
- The notification bell now opens a real notification center on desktop and
  mobile. The list is capped to the newest 50 notifications and indexed by
  `(user_id, created_at DESC, id DESC)` so accounts with large histories do not
  time out the shell.

Needed:

- Add a mobile-first shell layout that shows one navigation surface at a time:
  brand, menu button, account summary, and workspace selector without
  horizontal scrolling.
- Remove `/ops` from public/auth navigation and normal account menus, or gate it
  behind an explicit development/admin-only affordance.
- Add route-level shell rendering coverage across the shared ProductShell
  routes, including normal-user account menu behavior.
- Keep notification center coverage in the browser smoke set as new
  notification-producing workflows are added.

Checks:

- [x] At mobile widths, desktop nav and toolbar are hidden while the mobile menu
  is the only primary navigation control.
- [ ] `/session`, `/learn`, `/courses`, `/admin`, and `/settings/account` have
  no horizontal overflow at desktop or mobile widths.
- [ ] ProductShell tests cover the CSS module split by rendering active nav,
  disabled account actions, workspace controls, and mobile menu states.
- [ ] Public auth pages and account menus no longer show `/ops` to normal users.
- [x] The notification bell opens a real notification menu or route with unread
  count, list, empty, loading, error, mark-read, and clear states.
- [x] Notification helpers are covered by component/page tests and at least one
  browser smoke that marks a notification read.

## Admin Routes And Operations Replacement

Current evidence:

- The platform admin dashboard renders rich cards for teacher review, reward
  amount review, fraud controls, delegations, exports, wallets, and system
  status, but the cards are not links. The browser-visible links on `/admin`
  are only the shell links, account settings, and `/ops`.
- Direct navigation to `/admin/teacher-applications`,
  `/admin/rewards/amount-review`, `/admin/wallets`, `/admin/fraud-blocks`,
  `/admin/exports`, and `/admin/system` works, so the routes exist but are not
  discoverable from the admin dashboard.
- Direct navigation to `/admin/delegations` crashes into the Next.js error
  page. Browser console shows `TypeError: Cannot read properties of undefined
  (reading 'length')` in `AdminDelegationsRoute`.
- The frontend helper declares `fetchDelegations()` as returning
  `{ delegations, limit, offset, total }`, but the running backend
  `/api/delegated-permissions?limit=2` returns a raw array of delegation rows.
- `/admin/fraud-blocks` logs a React warning about duplicate/missing keys in
  `FraudBlockDetail` audit history.
- `/ops` still presents the old business console with many raw inputs,
  permission checkboxes, disabled actions, and API-result output. The product
  routes cover much of this work now, but `/ops` remains the discoverable
  fallback.

Needed:

- Fix the delegation route contract mismatch: either wrap the backend list
  response with pagination metadata or make `fetchDelegations()` and
  `AdminDelegationsRoute` consume the raw array safely.
- Add resilient empty/error states so a malformed admin API response cannot
  crash the whole route.
- Make admin dashboard cards and metrics navigate to their matching product
  routes.
- Fix the fraud-block audit list keys.
- Decide whether `/ops` is a development-only API console or a supported
  product surface, then remove it from normal navigation if it is development
  only.
- Keep any remaining ops-only action tracked as a product-route gap until there
  is no need for users to reach `/ops`.

Checks:

- [ ] `/admin/delegations` loads without a framework error overlay when the API
  returns zero rows, a raw array, or a paginated object.
- [ ] Delegation list, create, select, revoke, and permission-denied states are
  covered by tests using the actual backend response shape.
- [ ] `/admin` cards link to teacher applications, reward amount review,
  wallets, fraud blocks, delegations, exports, and system status.
- [ ] Browser console has no `AdminDelegationsRoute` runtime error and no
  `FraudBlockDetail` key warning after visiting admin subroutes.
- [ ] `/ops` is hidden from public/auth/account navigation unless an explicit
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
- Backend routes exist for learner progress, but learner and teacher UI copy
  still says lesson progress is local or not persisted.
- Assessment endpoints and helpers exist, but there is still no route surface
  for a learner to take assessments or for a teacher to author them.

Needed:

- Decide whether course learning is enrollment-gated or supports public
  previews, then make the route copy, API behavior, and navigation match that
  decision.
- Replace generated/test-like course titles in seeded/demo data used by the
  running product, or hide noisy seed data from normal catalog views.
- Wire persisted learner progress into `/courses/[courseId]/learn`, `/learn`,
  and teacher student views instead of showing "Local only" or "not tracked"
  states when backend progress routes are available.
- Add course catalog pagination or explicit "showing first N" controls if the
  catalog count is larger than the rendered list.
- Add assessment entry points inside learner course detail/learn routes and
  teacher course authoring routes.

Checks:

- [ ] A non-enrolled learner is either blocked from `/courses/[id]/learn` with a
  clear enrollment message or shown an intentional preview mode.
- [ ] Pending join requests cannot access gated lesson content unless preview
  mode is explicitly allowed.
- [ ] Persisted progress updates survive refresh and appear consistently on the
  learner dashboard, course learn route, and teacher student route.
- [ ] Catalog pages do not expose generated/test-like names in normal product
  smoke data.
- [ ] Large catalogs have pagination, load-more, or a clear visible count that
  matches the rendered set.
- [ ] Assessment links appear only when assessments exist and handle not
  started, in progress, passed, failed, max-attempts, and unpublished states.

## Teacher Course Workflow Gaps

Current evidence:

- `/teach/courses/1/content` can create a chapter, but content creation is
  disabled until a chapter exists. The page still says processing retry and
  destructive editing controls are separate until contracts are complete.
- `/teach/courses/1/enrollments` says persisted progress and reward eligibility
  are not available in the enrollment route.
- `/teach/courses/1/students` says lesson completion is not persisted yet and
  repeats that reward evidence is empty for each learner.
- `/teach/courses/1/rewards` is linked from the course workspace, but direct
  browser navigation lands on `permission_denied` with "User does not have
  reward candidate permission". The shell also loses normal workspace context
  while rendering the error state.
- The teacher course workflow mixes real actions, honest placeholder copy, and
  permission-gated dead ends, so users can click into a route that looks like a
  product feature but cannot be used from the visible role state.

Needed:

- Finish teacher content lifecycle controls: edit, unpublish/delete, upload
  retry/reprocess, processing-error inspection, and audit history.
- Wire persisted learner progress and reward eligibility into enrollment and
  student routes.
- Hide or explain course reward review links when the current teacher lacks the
  needed course reward-candidate permission.
- Preserve shell session/workspace context on permission-denied teacher routes.
- Add assessment-authoring routes alongside content authoring.

Checks:

- [ ] Content create/edit/delete/unpublish/retry flows have success, loading,
  validation, conflict, denied, and backend-error tests.
- [ ] Enrollment and student routes show persisted progress and reward
  eligibility from backend data after refresh.
- [ ] Course reward review links are visible only when the user can open the
  reward route, or they render a clear disabled state with the missing
  permission.
- [ ] `/teach/courses/[id]/rewards` denial keeps the normal signed-in shell,
  workspace selector, breadcrumbs, and account menu.
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

- [ ] A platform admin can search and open existing organizations from product
  navigation.
- [ ] A user with no organizations sees a clear create/request/access path,
  not only an empty state.
- [ ] Organization owners can invite/add members and see pending invite or join
  states from product routes.
- [ ] Organization routes preserve clear permission-denied, not-found, empty,
  loading, and backend-error states.
- [ ] Workspace selector options update after organization access is granted or
  revoked.

## Account Verification And Session Persistence

Current evidence:

- `/verify-email` provides token verification and a visible resend-verification
  form. `POST /api/auth/resend-verification` returns neutral account copy,
  rotates active tokens for unverified users, and prints the local mock email.
- Registration success tells local users to find the mock verification link in
  API logs, which is useful for development but not normal product copy.
- Login, registration, forgot/reset, and verify public nav no longer expose
  `/ops`.
- The frontend stores JWTs in `sessionStorage`, so a new browser tab or fresh
  in-app tab can look signed out while another tab is signed in. This explains
  the earlier "Sign in required" surprises.
- Account settings notification preferences now connect to a real shell
  notification center.

Needed:

- Add production email-provider delivery, bounce/error handling, and rate
  limiting for verification emails.
- Replace local-log instructions with a development-only hint or a real
  in-product verification delivery status.
- Decide whether auth should persist across tabs. If yes, move to a safer
  shared persistence strategy; if no, explain tab-scoped sessions clearly and
  avoid confusing redirects.
- Keep notification preferences connected to a real notification center.

Checks:

- [x] A newly registered user can request another verification email without
  exposing whether an address exists.
- [x] `/verify-email` handles valid, missing, malformed, expired, reused, and
  already-verified tokens.
- [ ] Registration success copy is user-facing in production and development
  hints are gated to local builds.
- [ ] Opening a new tab has an intentional, tested auth behavior with clear
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

Needed:

- Add platform user and role management routes with search, filters, role
  assignment, permission preview, audit history, and permission-denied states.
- Add teacher/organization course creation, course metadata editing, lifecycle
  transition, ownership/organization attachment, and publish/archive controls.
- Add reward policy management for platform, organization, and course scopes:
  create, list, inspect, activate/deactivate, validate coverage, and audit.
- Add reward-policy picker/search to fraud-block creation and reward
  operations wherever a numeric policy id is currently required.
- Add token-tax admin configuration with current values, proposed changes,
  validation, audit trail, and clear user-facing impact on deposit/retirement
  flows.

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
- [ ] Deposit and retirement tax configuration can be viewed and changed only by
  authorized admins, with audit rows and visible downstream wallet impact.
- [ ] Browser/page tests cover user-role management, course lifecycle changes,
  reward policy management, policy-backed fraud blocks, and token-tax updates.

## Page-Level Product Tests

Current evidence:

- `TODO/15-testing-gaps.md` still calls out zero page-level integration tests
  for `web/src/app/` pages.
- Component and API-helper tests exist, but product routes can still regress at
  the page boundary: URL params, redirects, storage/session setup, shell
  composition, route-level loading/error states, and browser navigation.

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
- [ ] The page-level test command is documented and runs without depending on
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
