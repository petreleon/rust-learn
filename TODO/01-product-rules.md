# Product Rules And Edge Cases

## Product Frontend Rules

- [ ] Build product screens for real people first: learners, teachers,
      organization operators, and platform administrators.
- [ ] Keep the current workflow console only as an internal operations/debug
      surface, ideally moved behind an `/ops` route and not treated as the main
      product UI.
- [ ] Stop relying on manual JWT fields, raw numeric ids, and permission
      checkboxes for normal user journeys.
- [ ] Keep UI authorization permission-driven. The frontend may present
      learner, teacher, organization, or admin workspaces, but affordances must
      still come from resolved platform, organization, course, or delegated
      permissions.
- [ ] Every primary screen must have useful loading, empty, error, denied,
      success, and retry states.
- [ ] Every checkpoint must be tested on desktop and mobile, then smoke-tested
      through Docker Compose and Kubernetes when the route is deployed there.

## Self-Critique Loop For Frontend Work

Before implementing or accepting each frontend checkpoint:

- [ ] Read the planned flow and ask: "Is this a real user journey, or just a
      nicer backend test harness?"
- [ ] Remove any normal-user requirement to paste JWTs, know internal ids, or
      toggle permissions by hand.
- [ ] Check whether the screen answers the user's next obvious question: what
      happened, what can I do now, and why is an action unavailable?
- [ ] Recheck mobile first-viewport usability before desktop polish.
- [ ] Verify the route with real API data or an explicit documented empty state.
- [ ] Repeat the critique until the route feels useful without explanation.

## Frontend Edge Cases And Invariants

- [ ] Treat the backend as the source of truth for identity, permissions,
      course membership, wallet state, reward state, and operation results.
- [ ] Never infer authorization from persona, route, menu item, local role
      label, or stale cached permission data.
- [ ] Handle users who belong to multiple organizations, teach multiple
      courses, learn in other courses, and hold temporary delegated permissions
      at the same time.
- [ ] Handle users with no organization, no enrolled courses, no wallet, no
      verified email, no active permissions beyond learner defaults, or only
      expired/revoked delegated permissions.
- [ ] Handle session expiry while a form is dirty, a modal is open, a file is
      uploading, a CSV is downloading, or a reward/admin action is pending.
- [ ] Handle backend responses that return JSON, CSV, or plain text error
      bodies. The UI must not assume every failure is JSON.
- [ ] Distinguish `401` unauthenticated, `403` permission denied, `404` missing
      resource, `409` conflict/invalid state, timeout, network failure, and
      backend `5xx` failures in user-facing copy.
- [ ] Make every mutation double-submit-safe in the UI and resilient to server
      idempotency conflicts.
- [ ] Preserve user input on validation errors, permission-denied responses,
      network failures, and route transitions where possible.
- [ ] Do not show raw internal ids as the primary way to choose users,
      organizations, courses, applications, rewards, blocks, or delegations.
      Use searchable pickers, contextual lists, or links from parent screens.
- [ ] Show audit/history context anywhere a decision can no longer be changed
      because the backend state has moved on.
- [ ] Keep product screens useful when the API has no records yet. Empty states
      should explain the next valid action and the permission needed for it.
- [ ] Treat async states as first-class: uploaded media can be unprocessed,
      processing, failed, retried, or complete; rewards can be pending,
      approved, rejected, token pending, wallet credited, reconciled, failed, or
      blocked.
- [ ] Keep destructive or irreversible actions behind explicit confirmation and
      post-action audit visibility.
- [ ] Keep CSV/export flows accessible on mobile and show download progress,
      empty exports, denied exports, and failed exports clearly.
- [ ] Do not leak sensitive details from admin-only or fraud workflows to
      learners, teachers, or organization users who lack the matching
      permission.
- [ ] Make route guards race-safe: a deep link should wait for current-user and
      permission resolution before redirecting or rendering a denied state.
- [ ] Keep Docker Compose and Kubernetes behavior equivalent for browser API
      roots, auth/session handling, downloads, uploads, health probes, and
      runtime log expectations.

