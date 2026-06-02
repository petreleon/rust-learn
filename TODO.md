# RustLearn Business Logic TODO

This TODO focuses on product/business behavior for RustLearn. Implementation
must use explicit platform, organization, and course permissions for access
decisions. Do not hard-code behavior by role name such as student, teacher,
moderator, admin, or super admin. Roles may remain permission bundles, but
business logic must ask "does this user have this permission in this scope?"

## Permission-First Rules

- [ ] Audit every existing service and endpoint that branches on role names and
      replace business decisions with platform, organization, or course
      permission checks.
- [ ] Keep role hierarchy checks only for role-assignment safety. Do not use
      hierarchy as a substitute for business authorization.
- [ ] Define missing business permissions before adding flows. Suggested
      platform permissions: `SUBMIT_TEACHER_APPLICATION`,
      `REVIEW_TEACHER_APPLICATIONS`, `APPROVE_TEACHER_APPLICATION`,
      `REJECT_TEACHER_APPLICATION`, `DELEGATE_REWARD_APPROVAL`,
      `SET_REWARD_POLICY`, `APPROVE_REWARD_AMOUNT`,
      `EXECUTE_REWARD_PAYOUT`, and `VIEW_REWARD_AUDIT`.
- [ ] Define suggested organization permissions:
      `NOMINATE_TEACHER_FOR_PLATFORM_REVIEW`,
      `VIEW_ORG_TEACHER_APPLICATIONS`, `SUBMIT_ORG_COURSE_REWARD_EVENT`,
      `VIEW_ORG_REWARD_REPORTS`, and `MANAGE_ORG_REWARD_BUDGET`.
- [ ] Define suggested course permissions:
      `CREATE_REWARDABLE_COURSE_EVENT`, `VIEW_COURSE_REWARD_STATUS`,
      `SUBMIT_COURSE_REWARD_EVENT`, `GRADE_REWARDABLE_ASSESSMENT`, and
      `MANAGE_COURSE_REWARD_RULES`.
- [ ] Update `PERMISSIONS.md`, permission constants, seed data, and tests
      whenever a business permission is added or renamed.

## Teacher Application Flow

- [ ] Add a platform-level teacher application entity with applicant user id,
      requested teaching scope, experience summary, optional organization
      sponsor, portfolio links, status, reviewer id, decision reason, and
      timestamps.
- [ ] Allow any authenticated user with `SUBMIT_TEACHER_APPLICATION` platform
      permission to apply to central administration.
- [ ] Allow organization-scoped nomination only through an organization
      permission such as `NOMINATE_TEACHER_FOR_PLATFORM_REVIEW`; nomination
      should create or attach to the same central application queue.
- [ ] Let central administration reviewers fetch, filter, approve, reject, or
      request changes only through platform permissions such as
      `REVIEW_TEACHER_APPLICATIONS`, `APPROVE_TEACHER_APPLICATION`, and
      `REJECT_TEACHER_APPLICATION`.
- [ ] On approval, assign the minimum needed platform, organization, or course
      permission bundle for the approved teaching scope. Do not assume the
      assignee is a teacher because of a role name.
- [ ] Record an immutable audit event for every teacher application transition.
- [ ] Send notifications to the applicant, organization sponsor when present,
      and central reviewers for submitted, changed, approved, and rejected
      applications.

## Course Business Flow

- [ ] Model course lifecycle states: draft, submitted, needs changes,
      approved, published, archived, and suspended.
- [ ] Gate course creation by `CREATE_COURSE` at platform or organization
      scope, depending on where the course is owned.
- [ ] Gate course editing by `MODIFY_COURSE` or `MANAGE_COURSE_SETTINGS` in
      the course scope.
- [ ] Gate course publication by `PUBLISH_CONTENT` or a dedicated
      `APPROVE_COURSE_PUBLICATION` permission, not by role name.
- [ ] Gate enrollment request, approval, removal, and waitlist behavior with
      `JOIN_COURSE`, `REQUEST_JOIN_COURSE`,
      `APPROVE_COURSE_JOIN_REQUESTS`, and `MANAGE_COURSE_ENROLLMENTS`.
- [ ] Keep content visibility and assessment access permission-based with
      `VIEW_CONTENT`, `VIEW_ASSESSMENT`, `TAKE_TESTS`, and related course
      permissions.

## Student Reward Flow

- [ ] Add rewardable course events for assessment completion, course
      completion, manually approved completion, and administrative adjustment.
- [ ] Restrict manual reward candidate submission for a course to users with
      `SUBMIT_COURSE_REWARD_EVENT` or `CREATE_REWARDABLE_COURSE_EVENT` in that
      exact course scope. This is the permission that should normally be
      granted to the course teacher permission bundle.
- [ ] Allow organization-level submission for courses attached to that
      organization only through `SUBMIT_ORG_COURSE_REWARD_EVENT`. This is the
      permission that should normally be granted to the organization admin
      permission bundle.
- [ ] Allow delegated moderators or operators to submit reward candidates only
      when they have a valid delegated permission covering the exact course or
      organization scope. Do not check the moderator role name.
- [ ] Treat student course activity as completion evidence, not as authority to
      submit a reward candidate. A student should receive rewards through an
      authorized course, organization, or delegated submission path.
- [ ] Store each reward candidate with a stable idempotency key such as
      `course_completion:{course_id}:{user_id}:{attempt_id}`.
- [ ] Add eligibility checks for enrollment, email verification, course policy,
      organization policy, anti-abuse limits, prior rewards, passing score, and
      completion percentage.
- [ ] Add versioned reward policies that define token amounts, multipliers,
      caps, cooldowns, and whether the policy pays from treasury transfer or
      token mint.
- [ ] Let an authorized reviewer decide the amount a student receives through
      `APPROVE_REWARD_AMOUNT` or equivalent delegated permission. The reviewer
      may be backed by an admin/moderator role bundle, but the code must check
      only the permission.
- [ ] Support delegated reward approval so central administration can grant a
      moderator or another operator permission to review reward amounts without
      changing reward service logic.
- [ ] Persist approved, rejected, adjusted, token pending, token confirmed,
      wallet credited, notified, completed, needs reconciliation, and failed
      reward states.
- [ ] Enqueue token execution only after eligibility and approved amount are
      recorded.
- [ ] Prefer treasury/presigner transfer when available; mint LearnToken only
      when policy explicitly allows supply expansion.
- [ ] Credit the internal wallet only after token confirmation unless the
      deployment is explicitly configured for off-chain-only rewards.
- [ ] Record external transaction hash, chain id, contract address, log index,
      recipient address, amount, and event type for every reward payout.
- [ ] Send reward notifications after wallet credit succeeds, including course
      context, approved amount, destination wallet, and transaction reference.
- [ ] Add reconciliation that repairs missing wallet credits, notifications, or
      transaction links from the last confirmed reward state without creating
      duplicate payouts.

## Wallet And Payment Business Flow

- [ ] Keep user and organization wallet linking idempotent and permission-based.
- [ ] Gate platform wallet operations with `MANAGE_WALLETS`,
      `VIEW_WALLET`, `VIEW_TRANSACTIONS`, `RECONCILE_WALLETS`, and related
      platform permissions.
- [ ] Gate organization wallet operations with `MANAGE_ORG_WALLETS`,
      `VIEW_ORG_REWARD_REPORTS`, and `MANAGE_ORG_REWARD_BUDGET`.
- [ ] Add wallet audit views that show internal transactions, external
      blockchain events, reward records, and reconciliation status.
- [ ] Add compensation records for manual reward corrections. Do not mutate
      historical reward decisions in place.

## Administration And Delegation

- [ ] Add a permission-delegation model that lets central administration grant
      a scoped permission to another user for a limited scope and optional
      expiration.
- [ ] Require `DELEGATE_REWARD_APPROVAL` or another explicit platform
      permission before a user can delegate reward approval ability.
- [ ] Store delegation reason, scope, expiration, grantor, grantee, and revoked
      status.
- [ ] Revoke delegated permissions without deleting the historical delegation
      record.
- [ ] Include delegated permissions in middleware authorization checks only
      when scope, expiration, and revocation state are valid.
- [ ] Log every delegated permission grant, use, and revocation.

## Reporting And Audit

- [ ] Add central administration dashboards for teacher applications, reward
      candidates, pending approval amounts, payout failures, and reconciliation
      mismatches.
- [ ] Add organization dashboards for sponsored teacher applications, course
      reward volume, approved amounts, and wallet balances.
- [ ] Add student-facing reward history with candidate status, approved amount,
      wallet credit, and token transaction reference.
- [ ] Export CSV reports for teacher applications, reward approvals, token
      payouts, wallet credits, and delegated permission activity.
- [ ] Ensure every report is protected by explicit platform, organization, or
      course reporting permissions.

## Data Model And API Work

- [ ] Add migrations for teacher applications, reward policies, reward
      candidates, reward decisions, reward execution jobs, reward audit events,
      and delegated permissions.
- [ ] Add repository/service layers for each business workflow. Keep complex
      business decisions out of Actix handlers.
- [ ] Add API endpoints for teacher applications, review decisions, reward
      policy management, reward amount approval, reward status, and delegated
      permission management.
- [ ] Add idempotency keys and unique constraints wherever a business event can
      be retried.
- [ ] Add structured logs for teacher application transitions, reward decisions,
      delegated permission checks, token execution, wallet credit, and
      reconciliation.

## Frontend Workflows

- [ ] Build the teacher application form and central review queue.
- [ ] Build reward approval screens that show eligibility evidence and let a
      permitted reviewer set or adjust the amount.
- [ ] Build student reward status/history screens.
- [ ] Build organization reward reporting screens.
- [ ] Build delegated-permission management screens for central administration.
- [ ] Keep UI affordances permission-driven. Hide or disable actions by
      resolved permissions, not by role labels.

## Tests And Verification

- [ ] Add permission-focused tests proving teacher application, reward amount
      approval, token execution, and wallet credit are allowed by permission and
      denied without permission.
- [ ] Add tests proving reward candidate submission succeeds for a user with
      `SUBMIT_COURSE_REWARD_EVENT` on the course, succeeds for a user with
      `SUBMIT_ORG_COURSE_REWARD_EVENT` on an organization attached to the
      course, succeeds for a user with a valid delegated scoped permission, and
      fails for everyone else.
- [ ] Add tests proving users with different role names but the same permission
      can perform the same business action.
- [ ] Add tests proving users with privileged role names but missing the
      required permission cannot perform the business action.
- [ ] Add idempotency tests for teacher applications, reward candidates, reward
      approval, token execution, wallet credit, and reconciliation.
- [ ] Add Docker Compose integration tests for PostgreSQL-backed reward flows,
      wallet credit, notifications, and Anvil token transaction recording.
- [ ] Keep Docker Compose verification commands documented for every business
      flow that requires PostgreSQL, RustFS, Anvil, or the worker.
