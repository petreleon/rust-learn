# Testing And Migration

## Frontend Testing And Runtime Verification

- [ ] Maintain the detailed QA checklist in
      [11-qa-matrix.md](11-qa-matrix.md) and use it for every milestone.
- [ ] Add route-level smoke tests for login, learner dashboard, course detail,
      teacher application, teacher dashboard, organization dashboard, and
      platform admin dashboard.
- [ ] Add interaction tests for permission-denied, empty, error, loading, and
      success states.
- [ ] Add contract tests for frontend API helpers covering JSON success, CSV
      success, text errors, `401`, `403`, `404`, `409`, timeout, network
      failure, and backend `5xx` responses.
- [ ] Add route-guard tests for expired session, missing current-user data,
      multi-organization membership, course-only permissions, and delegated
      permissions that are expired, revoked, or scoped elsewhere.
- [ ] Add at least one end-to-end learner path through Docker Compose:
      register or login, view course state, inspect rewards, and confirm logs
      stay clean.
- [ ] Add at least one end-to-end teacher path through Docker Compose:
      apply or manage course work, submit/approve a reward candidate, and
      confirm logs stay clean.
- [ ] Add at least one end-to-end platform admin path through Docker Compose:
      review teacher applications, approve reward amounts, inspect reports, and
      confirm logs stay clean.
- [ ] Add Kubernetes smoke checks that verify deployed product routes, not only
      health endpoints and operation-console smoke text.
- [ ] Keep `make preflight` green after every checkpoint.

## Migration Plan From Operations Console To Product UI

- [ ] Keep the current console working while product routes are introduced.
- [ ] Extract reusable request helpers and permission logic from the console
      before copying UI patterns into product pages.
- [ ] Build the app shell and authentication flow first.
- [ ] Build the learner dashboard and course discovery next because they define
      the main product identity.
- [ ] Build teacher application and teacher workspace after learner routes.
- [ ] Build organization and platform admin dashboards after shared table,
      filter, and detail components exist.
- [ ] Retire or demote console controls once equivalent product flows are
      verified.
- [ ] Use [13-milestone-06-ops-deprecation.md](13-milestone-06-ops-deprecation.md)
      for the detailed `/ops` migration and runtime smoke-test updates.

## Milestone Exit Gates

- [ ] Run the critique protocol in
      [00-self-criticism.md](00-self-criticism.md) before declaring the
      milestone implementation-ready.
- [ ] Run the recursive review in
      [12-recursive-review.md](12-recursive-review.md) when a milestone changes
      contracts, routing, permissions, runtime checks, or completion criteria.
- [ ] Each milestone has a desktop and mobile rendered check for the primary
      happy path and at least one denied, empty, and error state.
- [ ] Each milestone has a Docker Compose smoke path that exercises a real API
      request and confirms recent app/web/worker logs stay clean when relevant.
- [ ] Each deployed milestone has a Kubernetes smoke path for the product route,
      not only `/healthz` or operation-console text.
- [ ] Each milestone keeps the operations console usable until the equivalent
      product flow is verified.
- [ ] Each milestone updates README or `web/README.md` when routes,
      environment variables, auth/session behavior, or deployment expectations
      change.
