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
- [ ] If wallet budget actions do not explain permission, balance, and audit
      effects, reject the flow.

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
- [ ] Organization reward reports with filters, pagination, CSV export, payout
      failures, and reconciliation indicators.
  - [x] Existing all-time organization reward dashboard contract is now used by
        the frontend through
        `GET /api/reports/organizations/{organizationId}/reward-dashboard` and
        `.csv`. Date filters, pagination, payout-failure drill-downs, and
        reconciliation rows remain open backend contract work.
- [ ] Organization wallet audit and budget contract.

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
        Full organization health metrics remain open until backend contracts
        exist.
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
- [x] `/organizations/[organizationId]/reports` reward reports and exports.
  - [x] Shows all-time reward candidates, approved rewards, approved amount,
        sponsored teacher application summary, report-scoped wallet balance,
        course reward volume, CSV export, refresh, signed-out, denied, empty,
        backend-failure, and stale organization states without raw id entry.
        Date-filtered, paginated, payout-failure, reconciliation, and wallet
        audit details remain open until their contracts exist.
- [ ] `/organizations/[organizationId]/wallet` wallet, budget, and audit.
- [ ] `/organizations/[organizationId]/settings` scoped settings and
      permission-aware actions.

## Organization Dashboard

- [ ] Show organization health, member count, active courses, pending teacher
      applications, reward volume, wallet balance, and alerts.
- [x] Show quick actions only when scoped permissions allow them.
- [x] Explain denied quick actions with the missing scoped permission.
- [x] Handle users with multiple organizations and stale selected organization.
- [ ] Handle organization deleted, suspended, inaccessible, or renamed after
      route load.
  - [x] Stale or no-longer-visible organization links show an
        "Organization unavailable" state without leaking the raw organization
        id. Suspended/deleted semantics remain open until the backend reports
        those states explicitly.

## Member And Permission Management

- [x] Search, filter, and paginate members.
  - [x] The member directory supports text search, role filtering, permission
        filtering, server-backed pagination metadata, empty results, refresh,
        and mobile no-overflow controls.
- [ ] Invite users with clear pending, accepted, expired, and failed states.
- [ ] Assign organization roles only when hierarchy and scoped permissions
      allow it.
- [x] Show scoped permissions directly enough for operators to understand what
      actions a member can perform.
  - [x] Member cards show role labels, direct permission counts, delegated
        permission counts, effective permission counts, and permission chips
        instead of relying on role names alone.
- [ ] Confirm destructive removal and show audit/history when available.

## Reports And Wallet

- [ ] Date-filter organization reports and keep filters in URL state.
- [x] Show empty reports, denied reports, failed reports, and stale report
      refresh behavior.
  - [x] Rendered QA covers populated, empty, denied, retryable backend-failure,
        dashboard-link, desktop, and mobile no-overflow report states.
- [x] Download CSV with visible status, retry, and failure messaging.
  - [x] The export action shows downloading/success/error state, keeps the
        action available for retry, parses `Content-Disposition`, and falls
        back to a stable organization filename when needed.
- [ ] Show wallet balance, audit rows, reward credits, token links, and
      reconciliation indicators.
  - [x] Report-scoped wallet balance summary is visible to users with report
        access. Wallet audit rows, reward credits, token links, and
        reconciliation indicators remain open wallet/report contract work.
- [ ] Explain organization budget constraints before reward-related actions.

## Organization Edge Cases

- [x] User has member view but not invite/manage permissions.
  - [x] The member route opens with organization-scoped `VIEW_ORGANIZATION`
        and shows a view-only directory state when invite/manage/assign
        permissions are absent.
- [x] User has report permission but not wallet permission.
  - [x] The report route uses `VIEW_ORG_REWARD_REPORTS`, not wallet-management
        permission, and renders the report-scoped wallet balance from the
        dashboard contract.
- [x] User belongs to multiple organizations with different permissions.
- [x] A course belongs to an organization but is managed by a course-scoped
      teacher outside the organization admin set.
  - [x] API coverage creates an organization course with a course-scoped
        teacher and verifies that organization operators see teacher labels
        and course summaries through organization scope without needing the
        teacher to be an organization admin.
- [ ] Teacher nomination already exists or is already decided.
- [ ] CSV export is empty, slow, denied, or fails after request starts.
  - [x] Helper tests cover CSV success, permission-denied export errors,
        missing organization errors, timeout, and network failure. Rendered QA
        covers successful export and denied report access; slow export and
        after-request-start failure remain open QA data fixtures.
- [ ] Wallet exists but audit load fails.
- [ ] Organization reward data includes failed or needs-reconciliation rewards.

## Acceptance Evidence

- [ ] Desktop and mobile checks for organization dashboard, members, courses,
      reports, wallet, and denied state.
  - [x] Browser plus Playwright QA covers `/organizations` desktop
        multi-organization selector, delegated search/filter interaction,
        `/organizations/[organizationId]` capability dashboard, mobile
        selector first viewport/no-overflow, mobile no-organization denied
        state, and stale organization route without raw id leakage. Browser
        validated the main signed-in flow; Playwright supplied alternate
        session and mobile evidence after Browser could not reliably switch
        session storage.
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
- [ ] Docker Compose E2E path: organization dashboard, member/report action,
      CSV or wallet audit, log scan.
- [ ] Kubernetes smoke path loads organization routes and verifies selected
      workspace behavior with no console errors or horizontal overflow.
