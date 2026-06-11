# Platform Walkthrough After Refactor

Use this checklist to walk through the whole RustLearn platform after the recent
refactor, security hardening, and test cleanup work. The goal is to understand
what changed by exercising the product, not just reading the code.

## Preparation

- [ ] Read `AGENTS.md` and confirm the 180-line rule:
      manually maintained non-Markdown files stay at or below 180 lines;
      generated files, lockfiles, binary assets, and tool-owned artifacts are
      exempt.
- [ ] Read `README.md`, `PERMISSIONS.md`, and `TEST_SUMMARY.md`.
- [ ] Start the local stack with the documented development command.
- [ ] Confirm the admin password in local secrets is strong and not one of the
      old placeholder defaults.
- [ ] Confirm the database, RustFS/S3, Anvil, API, web app, and worker are all
      healthy.

## Recent Changes To Understand

- [ ] Review the large-file split strategy: root files now mostly aggregate
      smaller files with descriptive names.
- [ ] Confirm numbered file prefixes such as `01_`, `02_`, and `03_` are gone.
- [ ] Inspect a few split modules and verify the file names describe behavior,
      not ordering.
- [ ] Review the permission catalog consistency test and understand which
      sources it compares.
- [ ] Review bootstrap admin password validation and confirm weak/default
      passwords are rejected.
- [ ] Review the assessment attempts migration and understand the active-attempt
      uniqueness rule.
- [ ] Review Docker Compose and Kubernetes smoke test behavior.

## Authentication And Session Flow

- [ ] Register a new learner with a strong password.
- [ ] Try registering with a weak password and confirm it is rejected.
- [ ] After registration, find the local mock verification email printed by the
      API. In Docker Compose, inspect the API/app logs and look for
      `MOCK EMAIL`, `Confirm your RustLearn account`, or `verify-email?token=`.
- [ ] Copy the full verification URL from the mock email. The local default is
      an API URL shaped like
      `http://localhost:8080/api/auth/verify-email?token=<token>`.
- [ ] Open the copied verification URL in the browser, or paste only the token
      into the `/verify-email` page if you are testing the frontend route.
- [ ] Confirm the verification response says the email was verified, then retry
      login with the same account.
- [ ] Verify email and confirm login is blocked before verification.
- [ ] Log in after verification and inspect the current-session response.
- [ ] Confirm the session includes platform, organization, course, and delegated
      permission scopes when assigned.
- [ ] Log out and confirm protected routes reject unauthenticated access.

## Permissions And Roles

- [ ] Walk through platform role assignment as a super admin.
- [ ] Confirm lower-privileged users cannot assign higher roles.
- [ ] Test platform, organization, and course permission boundaries.
- [ ] Confirm custom roles can grant only the expected scoped permissions.
- [ ] Test delegated reward permissions and then revoke them.
- [ ] Confirm revoked delegated permissions stop authorizing actions.

## Learner Experience

- [ ] Browse the course catalog as a learner.
- [ ] Confirm published courses are visible and unrelated drafts are hidden.
- [ ] Request to join a course.
- [ ] Open the learner dashboard and verify enrolled, pending, and recommended
      courses.
- [ ] Open course learning content and verify locked/unlocked states.
- [ ] Submit progress and assessment attempts where applicable.
- [ ] Review learner reward history and wallet data.

## Teacher Experience

- [ ] Submit a teacher application.
- [ ] Re-submit with the same idempotency key and confirm idempotent behavior.
- [ ] Confirm duplicate open applications without the same key are rejected.
- [ ] Create or edit a course as a teacher with proper permissions.
- [ ] Add chapters and content.
- [ ] Publish course content and confirm notifications are created.
- [ ] Review join requests and approve, waitlist, or deny them.
- [ ] Submit and approve reward candidates where permissions allow.
- [ ] Confirm teachers without reward permissions cannot submit or approve
      reward candidates.

## Organization Experience

- [ ] Create or open an organization dashboard.
- [ ] Add a member by email.
- [ ] Assign organization roles and confirm hierarchy checks.
- [ ] Review organization member filters by role, permission, name, and email.
- [ ] Link courses to the organization.
- [ ] Review organization-scoped teacher applications.
- [ ] Nominate a teacher application from the organization context.
- [ ] Review organization reward dashboard data.
- [ ] Export organization reports and verify CSV contents.

## Platform Admin Experience

- [ ] Review platform dashboards for users, courses, rewards, fraud blocks, and
      teacher applications.
- [ ] Review and approve or reject teacher applications.
- [ ] Create and version reward policies.
- [ ] Confirm moderators have only the intended reward permissions.
- [ ] Review platform reward candidates and set approved amounts.
- [ ] Export platform reward, fraud, and delegated-permission reports.
- [ ] Confirm permission failures return clear `403` responses.

## Rewards, Wallets, And Blockchain

- [ ] Link a user wallet.
- [ ] Link an organization wallet.
- [ ] Create deposit and retire intents.
- [ ] Confirm platform-paid tax behavior for token operations.
- [ ] Execute reward payout planning for off-chain, token, and treasury policies.
- [ ] Confirm token confirmation records external transactions.
- [ ] Confirm wallet credit happens once and is idempotent.
- [ ] Review reconciliation flows and confirm missing side effects can be
      repaired without duplicate payouts.
- [ ] Create and revoke reward fraud blocks for teacher, organization, course,
      and policy scopes.
- [ ] Confirm active fraud blocks pause the expected reward activity.

## Video Upload And Worker

- [ ] Upload video content for a course.
- [ ] Confirm upload jobs are queued.
- [ ] Confirm the worker processes the job or records retry/failure state.
- [ ] Verify processed media URLs are available after processing.
- [ ] Confirm worker failure notifications are created when processing fails.

## API, Frontend, And Ops Checks

- [ ] Open the main frontend routes and confirm they render without console
      errors.
- [ ] Exercise learner, teacher, organization, admin, session, settings, and
      verification pages.
- [ ] Confirm frontend API helpers handle success, text errors, JSON errors,
      timeouts, `401`, and `403`.
- [ ] Run host Rust tests with `make test`.
- [ ] Run Docker Compose tests with `make test-compose`.
- [ ] Run integration tests with `make test-integration`.
- [ ] Run frontend tests, lint, and build from `web/`.
- [ ] Review Kubernetes manifests and dev overlay docs.
- [ ] Run Kubernetes smoke tests only when a cluster or port-forward target is
      available.

## Refactor Quality Review

- [ ] Pick one module from `src/api/`, `src/services/`, `src/repositories/`,
      `src/utils/`, `tests/`, and `web/`.
- [ ] Confirm each split file has a clear responsibility.
- [ ] Confirm aggregator files are easy to scan.
- [ ] Confirm no manually maintained non-Markdown file exceeds 180 lines.
- [ ] Confirm no `NN_` numbered split-file prefixes remain.
- [ ] Confirm renamed files are easier to navigate than the old numbered names.

## Notes To Capture While Walking Through

- [ ] Record any route that feels hard to discover.
- [ ] Record any permission error that is technically correct but confusing.
- [ ] Record any stale documentation or mismatch with the UI.
- [ ] Record any setup step that requires hidden local knowledge.
- [ ] Record any action that should have a stronger smoke test.
- [ ] Record any feature that needs clearer user-facing copy.

## Done Criteria

- [ ] You can explain the main user roles and their permissions.
- [ ] You can explain the learner, teacher, organization, and platform admin
      workflows.
- [ ] You can explain how rewards move from candidate to approval to payout to
      wallet/reconciliation.
- [ ] You can explain how the recent refactor changed file organization without
      changing product behavior.
- [ ] You have a short follow-up list of bugs, UX issues, documentation drift,
      and missing tests found during the walkthrough.
