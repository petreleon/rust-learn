# Testing And Migration

## Frontend Testing And Runtime Verification

- [ ] Maintain the detailed QA checklist in
      [11-qa-matrix.md](11-qa-matrix.md) and use it for every milestone.
- [ ] Add route-level smoke tests for login, learner dashboard, course detail,
      teacher application, teacher dashboard, organization dashboard, and
      platform admin dashboard.
- [ ] Add interaction tests for permission-denied, empty, error, loading, and
      success states.
  - [x] Mocked browser QA covers learner `/courses`, `/rewards`, `/wallet`,
        reward filter interaction, wallet unlinked empty state, wallet link
        success, and mobile menu interaction for this checkpoint.
  - [x] Mocked Playwright QA covers `/courses` catalog search/filter, detail
        navigation, request-join success, mobile menu/no-overflow, signed-out
        state, and `/courses/[courseId]` not-found state preserving the
        signed-in shell.
  - [x] Browser plus mocked Playwright QA covers
        `/courses/[courseId]/learn` signed-out state, authenticated desktop
        lesson switching, mobile no-overflow first viewport, failed media
        processing state, and learner content `403` preserving the signed-in
        shell.
  - [x] Browser plus mocked Playwright QA covers `/learn` authenticated
        dashboard, refresh interaction, mobile no-overflow first viewport,
        first-run empty state, and signed-out state.
  - [x] Browser plus mocked Playwright QA covers `/wallet` signed-out mobile
        state, linked desktop wallet credit history, unlinked mobile first
        viewport with link-wallet action visible, link-wallet success, and
        no horizontal overflow.
  - [x] Browser plus mocked Playwright QA covers `/settings/account`
        signed-out mobile state, linked desktop readiness, email-pending mobile
        next step, wallet-error mobile next step, verified unlinked mobile
        wallet next step, disabled notification defaults, and no horizontal
        overflow.
  - [x] Browser plus mocked Playwright QA covers `/teach/apply` signed-out
        state, authenticated desktop submission, stale duplicate `409`
        conflict refresh, needs-changes mobile feedback, rejected mobile
        reapply, hidden raw id entry, and no horizontal overflow.
  - [x] Browser plus mocked Playwright QA covers `/teach` and
        `/teach/courses` signed-out desktop/mobile states, authenticated
        desktop dashboard, mobile course search/lifecycle filtering that only
        reloads after apply, no-access state, plain-text backend `500` state,
        hidden internal operations console for non-admin product sessions, and
        no horizontal overflow.
  - [x] Browser plus mocked Playwright QA covers
        `/teach/courses/[courseId]` signed-out desktop/mobile states,
        course-list-to-workspace navigation, structured content outline,
        failed-processing state, scoped `403` state, hidden internal
        operations console for non-admin product sessions, absence of
        amount-review language, and no horizontal overflow.
  - [x] Browser plus mocked Playwright QA covers
        `/teach/courses/[courseId]/content` signed-out desktop/mobile states,
        authenticated course-workspace-to-content navigation, chapter
        creation, text/article content creation, mobile no-edit-permission
        disabled state, hidden internal operations console for non-admin
        product sessions, absence of amount-review language, and no horizontal
        overflow.
  - [x] Mocked Playwright QA covers
        `/teach/courses/[courseId]/enrollments` signed-out state,
        authenticated course-workspace-to-enrollment navigation, join-request
        approval, roster two-step removal, mobile permission-denied state,
        hidden internal operations console for non-admin product sessions,
        absence of amount-review language, and no horizontal overflow.
  - [x] Mocked Playwright QA covers
        `/teach/courses/[courseId]/students` signed-out state, authenticated
        course-workspace-to-students navigation, unsupported lesson-progress
        state, reward evidence display, mobile permission-denied state, hidden
        internal operations console for non-admin product sessions, absence of
        amount-review language, and no horizontal overflow.
  - [x] Browser plus Playwright QA covers
        `/teach/courses/[courseId]/rewards` signed-out state, authenticated
        course-workspace-to-rewards navigation, teacher approval interaction,
        status filter/all-status fallback learner display, stale `409`
        conflict refresh copy, mobile permission-denied state, hidden internal
        operations console for non-admin product sessions, absence of
        amount-review language, hidden raw learner/candidate ids, and no
        horizontal overflow. Browser screenshot capture and mobile text entry
        were unavailable in this runtime, so Playwright supplied screenshot and
        mobile denied evidence.
  - [x] Browser plus Playwright QA covers `/organizations` signed-out state,
        authenticated desktop multi-org selector, delegated organization
        search/filter interaction, role-only organization visibility,
        `/organizations/[organizationId]` permission dashboard, disabled
        contract-pending action buttons, missing scoped permission copy, stale
        organization route without raw id leakage, mobile selector
        first-viewport/no-overflow, and mobile no-organization denied state.
        Browser validated the main login and signed-in selector/dashboard
        flow; Playwright supplied alternate session and mobile evidence after
        Browser could not reliably switch session storage for no-org state.
  - [x] Playwright QA covers `/organizations/[organizationId]/reports`
        populated desktop report, CSV export/download status, dashboard
        Open-reports navigation, mobile no-overflow layout, missing report
        permission, empty report data, and backend `500` retry state. BrowserMCP
        returned `Transport closed` for this runtime, so Playwright supplied
        the rendered evidence.
  - [x] Playwright QA covers `/organizations/[organizationId]/courses`
        populated desktop course list, dashboard Open-courses navigation,
        search/lifecycle/reward filters, mobile no-overflow layout, missing
        course-list permission, empty course data, and backend `500` retry
        state. BrowserMCP returned `Transport closed` for this runtime, so
        Playwright supplied the rendered evidence.
- [ ] Add contract tests for frontend API helpers covering JSON success, CSV
      success, text errors, `401`, `403`, `404`, `409`, timeout, network
      failure, and backend `5xx` responses.
  - [x] Frontend helper tests now cover session/auth JSON and text errors plus
        learner reward-history success/filtering, wallet `404`, wallet link
        success, and learner `403` text error normalization.
  - [x] Frontend helper tests cover learner course catalog filters, course
        detail success, request-join success, and course-detail text `404`.
  - [x] Rust API tests cover learner course catalog/detail response shape,
        visibility, pending state, own draft access, and hidden unscoped draft
        detail.
  - [x] Frontend helper tests cover learner course-learning success and
        permission text-error normalization.
  - [x] Rust API tests cover learner course-learning response shape, active
        content selection, media processing state mapping, and unscoped learner
        content `403`.
  - [x] Frontend helper tests cover learner dashboard aggregation across
        enrolled catalog, recommended catalog, recent rewards, and unlinked
        wallet `404`.
  - [x] Frontend helper tests cover learner wallet aggregation across
        `GET /api/wallets/me` and recent reward credit history.
  - [x] Frontend helper tests cover self-wallet plain text backend `500`
        normalization used by account wallet-error UI.
  - [x] Frontend helper tests cover teacher application self-snapshot success,
        empty snapshot, submit body shape, plain text `409`, backend `500`,
        timeout, and network failure normalization.
  - [x] Rust API/service tests cover teacher application current-user
        snapshots, duplicate open conflicts, idempotency replay, rejected
        reapply, and route registration for `GET /api/teacher-applications/me`.
  - [x] Frontend helper tests cover teaching-course dashboard filters,
        response parsing, and plain-text backend failure normalization.
  - [x] Rust API tests cover `GET /api/courses/teaching` route registration,
        course-scoped teacher visibility, outsider empty results, lifecycle
        status, content summary, roster counts, reward queue counts, and action
        permission booleans.
  - [x] Frontend helper tests cover teaching-course workspace detail parsing
        plus plain-text `403` and `404` normalization.
  - [x] Rust API tests cover `GET /api/courses/teaching/{courseId}` route
        registration, scoped workspace response shape, teacher roles,
        publication summary, chapter/content structure, inherited publication
        status, display state, and outsider `403`.
  - [x] Frontend helper tests cover teacher chapter/content authoring request
        bodies plus permission and invalid-chapter error normalization.
  - [x] Rust API tests cover content endpoint chapter/course ownership
        guardrails so mismatched path chapters cannot be listed or mutated
        through another course.
  - [x] Frontend helper tests cover teacher enrollment workspace filters,
        decision request bodies, roster removal, permission errors, and
        missing-enrollment text-error normalization.
  - [x] Rust API tests cover `GET /api/courses/teaching/{courseId}/enrollments`
        response shape, default open filtering, pending-only filtering,
        learner context, roster access state, unsupported progress/reward
        eligibility flags, and outsider `403`.
  - [x] Frontend helper tests cover teacher student-progress parsing,
        unsupported progress fields, reward evidence, permission errors, and
        missing-course text-error normalization.
  - [x] Rust API tests cover `GET /api/courses/teaching/{courseId}/students`
        response shape, enrolled learners, unsupported progress flags, content
        totals, reward evidence counts/latest status, and outsider `403`.
  - [x] Frontend helper tests cover teacher reward-candidate status filters,
        response parsing, teacher decision request bodies, permission errors,
        stale `409` conflict text errors, and missing-candidate `404`
        normalization.
  - [x] Frontend helper tests cover organization workspace summaries,
        capability derivation, delegated organization access, role-only
        membership visibility, delegated search/filtering, and stale
        organization lookup.
  - [x] Frontend helper tests cover organization reward report dashboard JSON,
        CSV body and filename parsing, permission-denied and missing-report
        text errors, timeout, and network failure normalization.
  - [x] Frontend helper tests cover organization course-list filters,
        successful operator summaries, permission-denied text errors, missing
        organization, backend `5xx`, timeout, and network failure
        normalization.
  - [x] Rust API tests cover `GET /api/organizations/{organizationId}/courses`
        route behavior for organization-scoped access, search/reward filters,
        teacher/content/roster/reward summaries, permission booleans, and
        outsider `403`.
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
