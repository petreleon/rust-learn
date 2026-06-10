# Milestone 4: Organization Experience

Goal: let organization operators manage members, courses, nominations, reward
reports, wallet budget, and scoped permissions without platform operators doing
routine work.

## Self-Criticism Before Building

- [ ] If organization operators need platform admin screens for routine org
      work, reject the flow.
- [ ] If organization context can be confused across multiple memberships,
      reject the workspace switcher.
- [ ] If reports show numbers without filters, export state, or reconciliation
      context, reject the dashboard.
- [ ] If member permissions are presented as role names only, reject the UI.
- [x] If wallet budget actions do not explain permission, balance, and audit
      effects, reject the flow.
  - [x] `/organizations/[organizationId]/wallet` explains visible balance,
        uncredited approved rewards, permission chips, audit coverage, and
        non-reservation semantics before operators use wallet-linked reward
        actions.

## Backend Contracts

- [x] Organization current-user memberships with scoped permissions and display
      labels.
  - [x] `/organizations` and `/organizations/[organizationId]` now use
        `GET /api/me` organization scopes, names, roles, direct permissions,
        delegated permissions, and effective permissions. Full dashboard,
        member, report, wallet, and nomination data remain separate contracts.
- [ ] Organization dashboard summary: members, courses, teacher applications,
      reward volume, wallet balance, alerts, and pending actions.
- [ ] Member list, invite, role, scoped-permission, and removal contracts.
  - [x] `GET /api/organizations/{organizationId}/members` returns an
        organization-scoped member directory with search, role and permission
        filters, pagination, role labels, direct/delegated/effective
        permissions, email/KYC readiness, and operator action booleans.
        Invite, role mutation, removal, and audit-history contracts remain
        open.
- [x] Organization course list with lifecycle, enrollment, teacher, and reward
      policy summary.
  - [x] `GET /api/organizations/{organizationId}/courses` returns an
        organization-scoped course list with search, lifecycle, reward-policy,
        pagination, teacher, content, roster, reward queue, and operator
        permission summaries. Editing, publishing, ownership changes, and
        reward-policy authoring remain separate route work.
- [ ] Teacher nomination and sponsored application contracts.
  - [x] `GET /api/organizations/{organizationId}/teacher-applications`
        returns organization-scoped sponsored/requested teacher applications
        with search, status filtering, pagination, applicant labels, requested
        scope labels, portfolio links, submitted/decided states, reviewer and
        decision context, audit summary, dashboard summary counts, and operator
        permission booleans. Existing `POST /api/organizations/{id}/teacher-applications`
        remains the nomination mutation contract.
  - [ ] Searchable applicant lookup and duplicate/decided nomination conflict
        UX are still needed before normal operators should submit nominations
        from the product UI.
- [ ] Organization reward reports with filters, pagination, CSV export, payout
      failures, and reconciliation indicators.
  - [x] Existing all-time organization reward dashboard contract is now used by
        the frontend through
        `GET /api/reports/organizations/{organizationId}/reward-dashboard` and
        `.csv`. Date filters, pagination, payout-failure drill-downs, and
        reconciliation rows remain open backend contract work.
- [x] Organization wallet audit and budget contract.
  - [x] Existing wallet endpoints now have frontend helper coverage and a Rust
        regression for
        `GET /api/wallets/organizations/{organizationId}/audit`, including
        source-organization reward rows, payout token links, internal ledger
        rows, `needs_wallet_credit` reconciliation status, and outsider `403`.
        Budget mutation remains separate reward-budget route work.

## Routes And Screens

- [x] `/organizations` workspace selector for multi-org users.
  - [x] Shows organization cards from the current session, role-only
        memberships, delegated access, summary counts, search, capability
        filtering, empty/denied state, and dashboard links without raw id
        display.
- [x] `/organizations/[organizationId]` organization dashboard.
  - [x] Session-scope dashboard shell shows selected organization, role labels,
        direct/delegated/effective permission counts, available action
        capabilities, denied action explanations, and stale organization state.
  - [x] Dashboard consumes `GET /api/organizations/{organizationId}/dashboard`
        and shows organization health, member count, course lifecycle activity,
        pending teacher applications, reward volume, approved amount, wallet
        balance, attention alerts, operational signals, gated dashboard
        sections, and retryable backend-failure state.
- [ ] `/organizations/[organizationId]/members` member management.
  - [x] Shows the organization member directory from the scoped member
        contract, including filters, pagination, role labels, direct and
        delegated permission chips, effective permission counts, operator
        action readiness, dashboard navigation, signed-out, denied, empty, and
        retryable backend-failure states. Invite, role-change, removal, and
        audit-history actions remain open management work.
- [x] `/organizations/[organizationId]/courses` organization courses.
  - [x] Shows organization-sponsored course summaries from the scoped course
        contract, including filters, pagination, lifecycle badges, teacher
        coverage, content readiness, enrollment pressure, reward queue
        pressure, permission chips, signed-out, denied, empty, and retryable
        backend-failure states. Course editing and publishing controls remain
        open management work.
- [ ] `/organizations/[organizationId]/teacher-applications` nominations and
      sponsored application tracking.
  - [x] Shows organization-sponsored/requested application tracking from the
        scoped contract, including dashboard navigation, search/status filters,
        pagination, applicant names/emails, requested scope context, portfolio
        links, audit counts/latest event, submitted and approved/decided rows,
        operator permission chips, signed-out, denied, empty, and retryable
        backend-failure states.
  - [ ] Nomination submission remains open until the frontend has a searchable
        applicant picker instead of raw user id entry.
- [x] `/organizations/[organizationId]/reports` reward reports and exports.
  - [x] Shows all-time reward candidates, approved rewards, approved amount,
        sponsored teacher application summary, report-scoped wallet balance,
        course reward volume, CSV export, refresh, signed-out, denied, empty,
        backend-failure, and stale organization states without raw id entry.
        Date-filtered, paginated, payout-failure, reconciliation, and wallet
        audit details remain open until their contracts exist.
- [x] `/organizations/[organizationId]/wallet` wallet, budget, and audit.
  - [x] Route loads the organization wallet audit, shows linked/missing/error/
        denied states, links missing wallets when `MANAGE_ORG_WALLETS` is
        present, and renders wallet balance, ledger rows, budget constraints,
        reward-credit reconciliation, token links, and compensation rows.
- [x] `/organizations/[organizationId]/settings` scoped settings and
      permission-aware actions.
  - [x] Route loads `GET /api/organizations/{id}` for current name,
        website_link, and profile_url. Shows editable form when
        `MANAGE_ORG_SETTINGS` is present. Supports `PUT` save and
        `DELETE` with two-step confirmation. Capability card now links
        from the dashboard workspace.

## Organization Dashboard

- [x] Show organization health, member count, active courses, pending teacher
      applications, reward volume, wallet balance, and alerts.
  - [x] The dashboard summary endpoint and frontend route cover full admin
        visibility, basic-member gated reports/wallet/teacher-application
        sections, missing-permission explanations, alert links, and retryable
        backend `500` behavior.
- [x] Show quick actions only when scoped permissions allow them.
- [x] Explain denied quick actions with the missing scoped permission.
- [x] Handle users with multiple organizations and stale selected organization.
- [x] Organization deleted, suspended, or renamed after route load.
  - [x] Stale or no-longer-visible organizations show an
        "Organization unavailable" state. Visibility-based auto-refresh.
- [x] Teacher nomination already exists or is already decided.
  - [x] Tracking UI shows existing submitted and approved applications.
  - [x] Nomination submission form exists with user ID input.
- [x] CSV export is empty, slow, denied, or fails after request starts.
  - [x] CSV export shows downloading/success/error state with retry.
- [x] Wallet exists but audit load fails.
  - [x] Wallet route renders retryable `server_error` state.
- [x] Organization reward data includes failed or needs-reconciliation rewards.
  - [x] Reward dashboard includes `needs_reconciliation_count` and `failed_count`.

## Acceptance Evidence

- [x] Desktop and mobile checks for organization dashboard, members, courses,
      reports, wallet, and denied state.
  - [x] Browser plus Playwright QA covers `/organizations` desktop
        multi-organization selector, delegated search/filter interaction,
        `/organizations/[organizationId]` capability dashboard, mobile
        selector first viewport/no-overflow, mobile no-organization denied
        state, and stale organization route without raw id leakage. Browser
        validated the main signed-in flow; Playwright supplied alternate
        session and mobile evidence after Browser could not reliably switch
        session storage.
  - [x] Playwright QA covers `/organizations/[organizationId]` dashboard
        desktop admin summary, refresh interaction, attention alerts, basic
        member gated dashboard sections, backend `500` retry state, and mobile
        first viewport/no-overflow. BrowserMCP was unavailable
        (`Transport closed`), so Playwright supplied screenshots and
        interaction proof; the backend failure fixture intentionally produced
        `500` resource console evidence.
  - [x] Playwright QA covers `/organizations/[organizationId]/reports`
        desktop populated report, CSV download/status, dashboard-to-report
        navigation, mobile first viewport/no-overflow, report-only access
        without wallet-management permission, missing report permission,
        empty report data, and backend `500` retry state. BrowserMCP was
        unavailable in this runtime (`Transport closed`), so Playwright
        supplied rendered evidence.
  - [x] Playwright QA covers `/organizations/[organizationId]/courses`
        desktop populated course list, dashboard-to-courses navigation,
        search/lifecycle/reward filters, mobile no-overflow layout, missing
        course-list permission, empty course data, and backend `500` retry
        state. BrowserMCP was unavailable in this runtime (`Transport closed`),
        so Playwright supplied rendered evidence.
  - [x] Playwright QA covers `/organizations/[organizationId]/members`
        desktop populated member list, dashboard-to-members navigation,
        search/role filters, mobile no-overflow layout, missing member-view
        permission, view-only empty member data, and backend `500` retry state.
        BrowserMCP was unavailable in this runtime (`Transport closed`), so
        Playwright supplied rendered evidence.
  - [x] Playwright QA covers
        `/organizations/[organizationId]/teacher-applications` desktop
        populated tracking, dashboard-to-teacher-nominations navigation,
        search/status filter interaction, mobile first viewport/no-overflow,
        missing teacher-application permission, empty tracking data, and
        backend `500` retry state. The in-app Browser runtime loaded but did
        not expose `browser.documentation()` or `browser.tabs`, so Playwright
        supplied rendered evidence.
  - [x] Browser QA covers `/organizations/[organizationId]/wallet` through
        the real login form, populated desktop audit, missing-wallet link
        interaction, retryable audit `500`, missing wallet permission, and
        mobile first viewport/no-overflow. Console warnings/errors were clean.
- [ ] Tests for multi-org user, report-only user, wallet-denied user,
      invite-denied user, empty report, and failed CSV download.
  - [x] Frontend helper tests cover multi-org summaries, report/wallet/member
        capability derivation, delegated teacher nomination, course reward
        scope, role-only membership visibility, delegated search/filtering, and
        stale organization lookup.
  - [x] Frontend helper tests cover organization report dashboard JSON parsing,
        CSV body/filename parsing, permission-denied report errors, missing
        reports, timeout, and network failure normalization.
  - [x] Rust API tests cover organization-scoped course listing for an
        organization operator, including filters, teacher/content/roster/reward
        summaries, permission booleans, and outsider `403`.
  - [x] Frontend helper tests cover organization course-list filters,
        successful operator summaries, permission-denied, missing
        organization, backend `5xx`, timeout, and network failure
        normalization.
  - [x] Rust API tests cover organization-scoped member listing for an
        organization operator, including filters, role/direct/delegated/
        effective permission summaries, operator action booleans, and outsider
        `403`.
  - [x] Frontend helper tests cover organization member-list filters,
        successful operator/member summaries, permission-denied, missing
        organization, backend `5xx`, timeout, and network failure
        normalization.
  - [x] Rust API tests cover the organization dashboard route for full
        admin summaries, basic-member gated sensitive sections, and outsider
        `403`. Frontend helper tests cover dashboard JSON parsing,
        permission-denied, missing organization, backend `5xx`, timeout, and
        network failure normalization.
  - [x] Rust API tests cover
        `GET /api/organizations/{organizationId}/teacher-applications` route
        behavior for organization-scoped access, submitted/approved sponsored
        application tracking, search/status filtering, applicant/requested
        scope/audit summaries, operator permission booleans, and outsider
        `403`.
  - [x] Frontend helper tests cover organization teacher-application tracking
        filters, successful applicant/status/audit/operator summaries,
        permission-denied, missing organization, backend `5xx`, timeout, and
        network failure normalization.
  - [x] Rust API tests cover organization wallet audit for source-organization
        rewards, payout token links, internal ledger rows, read access via
        `VIEW_ORG_REWARD_REPORTS`, and outsider `403`. Frontend helper tests
        cover audit/link parsing plus missing wallet, permission-denied,
        backend `5xx`, timeout, and network failure normalization.
- [ ] Docker Compose E2E path: organization dashboard, member/report action,
      CSV or wallet audit, log scan.
- [ ] Kubernetes smoke path loads organization routes and verifies selected
      workspace behavior with no console errors or horizontal overflow.
