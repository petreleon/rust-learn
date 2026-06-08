# Milestone 6: Operations Console Deprecation

Goal: move the existing workflow console to an internal `/ops` route, keep it
useful for debugging, and stop treating it as the main frontend once product
routes are verified.

## Self-Criticism Before Building

- [ ] If `/` still opens the operations console after product routes exist,
      reject the migration.
- [ ] If `/ops` is removed before equivalent product flows are verified, reject
      the migration.
- [ ] If operations controls remain mixed into learner, teacher, organization,
      or admin navigation, reject the shell.
- [ ] If smoke tests still search only for operations-console text, reject the
      verification.
- [ ] If `/ops` exposes sensitive controls without appropriate session and
      permission handling, reject the route.

## Migration Tasks

- [x] Move the current single-page workflow console into `/ops`.
- [ ] Keep `/ops` behind authentication and permission checks appropriate for
      internal operators.
- [ ] Preserve the API root control only where it is still useful for local
      debugging.
- [ ] Remove manual permission toggles from product routes; keep them only in
      `/ops` if they remain valuable as a test harness.
- [x] Replace the default `/` route with product-home routing driven by
      current-user state.
- [x] Update smoke text constants so runtime checks prove product routes, not
      only `/ops`.
- [x] Update `web/README.md`, root `README.md`, and `TODO/` when the default
      frontend route changes.

## Product Flow Replacement Checklist

Do not demote a console control until its product equivalent is verified.

- [ ] Session panel replaced by login, registration, logout, current-user, and
      account routes.
- [ ] Permission checklist replaced by resolved permission state and
      permission-aware navigation.
- [ ] Teacher application controls replaced by learner/teacher application and
      platform review routes.
- [ ] Reward candidate controls replaced by teacher course reward review and
      platform amount review routes.
- [ ] Student reward history control replaced by learner reward history route.
- [ ] Organization report controls replaced by organization reports route.
- [ ] Fraud block controls replaced by platform fraud route.
- [ ] Delegation controls replaced by platform delegation route.
- [ ] CSV export controls replaced by organization/platform export routes.
- [ ] Result panel replaced by route-local status, toast, audit, and error
      states.

## Runtime And Test Updates

- [ ] Update Docker Compose runtime smoke checks to load product routes and
      call real API endpoints through the web proxy.
- [ ] Update Kubernetes runtime smoke checks to load product routes and verify
      web-to-API readiness from the deployed web pod.
- [ ] Keep a separate `/ops` smoke check only if the console remains operational
      infrastructure.
- [ ] Add regression tests proving product routes do not expose `/ops` controls
      to normal learners.
- [ ] Add regression tests proving `/ops` does not become the default route for
      authenticated product users.
- [ ] Scan recent app, web, and worker logs after the route migration.

## Edge Cases

- [ ] Existing bookmarks to `/` during migration.
- [ ] Existing bookmarks to `/#teacher-workflow`, `/#reward-workflow`, or
      other console anchors.
- [ ] Product user without `/ops` permission tries to open `/ops`.
- [ ] Operator opens `/ops` on mobile for emergency debugging.
- [ ] API root differs between local dev, Docker Compose, and Kubernetes.
- [ ] `/ops` action fails with text error while product routes use normalized
      helper responses.
- [ ] Console result state becomes stale after product routes mutate the same
      backend data.

## Acceptance Evidence

- [ ] Desktop and mobile rendered checks for `/`, core product routes, and
      `/ops`.
- [x] Browser proof that `/` no longer renders the operations console once
      product home exists.
- [x] Browser proof that `/ops` still renders meaningful debug controls for
      permitted operators.
- [ ] Docker Compose runtime verification uses product route smoke text.
- [ ] Kubernetes runtime verification uses product route smoke text.
- [ ] Recent logs are clean after route migration.
