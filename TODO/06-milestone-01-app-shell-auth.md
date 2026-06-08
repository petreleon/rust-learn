# Milestone 1: App Shell And Auth

Goal: replace the test-console entry experience with a real product shell,
normal authentication screens, current-user loading, and permission-aware
navigation.

## Self-Criticism Before Building

- [ ] If the first screen still feels like an API console, reject the design.
- [ ] If normal users must paste JWTs or toggle permissions, reject the flow.
- [ ] If navigation is based on role labels instead of resolved permissions and
      scopes, reject the implementation.
- [ ] If a deep link flashes unauthorized content before current-user loading
      finishes, reject the route guard.
- [ ] If mobile users cannot see where they are, who they are, and what to do
      next in the first viewport, reject the layout.

## Backend Contracts

- [x] Add or confirm `GET /api/me` or equivalent current-session endpoint.
- [x] Include user id, name, email, email verification status, platform
      permissions, organization memberships, course enrollments, delegated
      permissions, and useful display labels.
- [x] Include enough scope metadata for workspace switching without raw id
      entry.
- [x] Define unauthorized, expired-token, unverified-email, and missing-user
      response shapes.
- [x] Decide session storage strategy and document security tradeoffs in
      `web/README.md`.

## Routes

- [ ] `/` routes to the appropriate product home after current-user resolution,
      or to public course discovery when anonymous browsing is supported.
- [x] `/login` supports email/password login, invalid credentials, unverified
      email, server failure, timeout, and redirect back to intended route.
- [ ] `/register` supports password policy feedback, duplicate email, mock
      email-verification explanation, and post-register next step.
- [ ] `/verify-email` supports valid token, expired token, invalid token, and
      already-verified states.
- [ ] `/forgot-password` and `/reset-password` exist as product-ready shells or
      are explicitly deferred with non-broken navigation.
- [ ] `/settings/account` shows profile, email verification, wallet status, and
      notification preference placeholders.
- [ ] `/ops` hosts the existing operations console once it is moved out of the
      product home.

## App Shell

- [ ] Add persistent top-level layout with product identity, primary nav,
      workspace switcher, account menu, notification entry, and route title.
- [ ] Add mobile navigation that does not hide primary actions behind tiny text
      or overflow-prone controls.
- [ ] Add breadcrumbs or contextual back links for nested learner, teacher,
      organization, and platform admin routes.
- [ ] Add route-level loading skeletons that preserve layout stability.
- [ ] Add global toast/status area for success, retryable errors, session
      expiry, and background operation notices.

## Permission And Workspace Navigation

- [ ] Show learner navigation when the user has learner defaults or course
      enrollments.
- [ ] Show teacher navigation only when course permissions, approved teaching
      scope, or teacher application state justify it.
- [ ] Show organization navigation only for organization memberships or
      organization-scoped permissions.
- [ ] Show platform admin navigation only for platform permissions.
- [ ] Show delegated-permission affordances with expiration and scope context.
- [ ] Add denied-state pages that explain the missing permission without
      exposing sensitive admin details.

## Edge Cases

- [ ] User logs out with unsaved form state.
- [ ] Token expires while current-user request is in flight.
- [ ] Token expires after route data loads but before a mutation.
- [ ] User has no memberships and no courses.
- [ ] User has only course-scoped permissions and no organization membership.
- [ ] User has multiple organizations and a stale workspace selection.
- [ ] User has delegated permission that expires during the session.
- [ ] API root differs between local browser, Docker Compose, and Kubernetes.
- [ ] Browser refresh on a deep link preserves intended route after auth.

## Acceptance Evidence

- [ ] `npm run lint` and `npm run build` pass in `web/`.
- [ ] Desktop browser check covers login, current-user loading, navigation, and
      denied state.
- [ ] Mobile browser check covers login, nav open/close, workspace switcher,
      account menu, and denied state without horizontal overflow.
- [ ] API-helper tests cover JSON success, text errors, `401`, `403`, timeout,
      and network failure.
- [ ] Docker Compose smoke path logs in or loads a seeded session, hits a real
      API route, and scans recent app/web logs.
- [ ] Kubernetes smoke path loads the product route and verifies no framework
      overlay, no console errors, and correct API root behavior.
