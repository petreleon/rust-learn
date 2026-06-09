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
- [ ] Organization course list with lifecycle, enrollment, teacher, and reward
      policy summary.
- [ ] Teacher nomination and sponsored application contracts.
- [ ] Organization reward reports with filters, pagination, CSV export, payout
      failures, and reconciliation indicators.
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
- [ ] `/organizations/[organizationId]/courses` organization courses.
- [ ] `/organizations/[organizationId]/teacher-applications` nominations and
      sponsored application tracking.
- [ ] `/organizations/[organizationId]/reports` reward reports and exports.
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

- [ ] Search, filter, and paginate members.
- [ ] Invite users with clear pending, accepted, expired, and failed states.
- [ ] Assign organization roles only when hierarchy and scoped permissions
      allow it.
- [ ] Show scoped permissions directly enough for operators to understand what
      actions a member can perform.
- [ ] Confirm destructive removal and show audit/history when available.

## Reports And Wallet

- [ ] Date-filter organization reports and keep filters in URL state.
- [ ] Show empty reports, denied reports, failed reports, and stale report
      refresh behavior.
- [ ] Download CSV with visible status, retry, and failure messaging.
- [ ] Show wallet balance, audit rows, reward credits, token links, and
      reconciliation indicators.
- [ ] Explain organization budget constraints before reward-related actions.

## Organization Edge Cases

- [ ] User has member view but not invite/manage permissions.
- [ ] User has report permission but not wallet permission.
- [x] User belongs to multiple organizations with different permissions.
- [ ] A course belongs to an organization but is managed by a course-scoped
      teacher outside the organization admin set.
- [ ] Teacher nomination already exists or is already decided.
- [ ] CSV export is empty, slow, denied, or fails after request starts.
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
- [ ] Tests for multi-org user, report-only user, wallet-denied user,
      invite-denied user, empty report, and failed CSV download.
  - [x] Frontend helper tests cover multi-org summaries, report/wallet/member
        capability derivation, delegated teacher nomination, course reward
        scope, role-only membership visibility, delegated search/filtering, and
        stale organization lookup.
- [ ] Docker Compose E2E path: organization dashboard, member/report action,
      CSV or wallet audit, log scan.
- [ ] Kubernetes smoke path loads organization routes and verifies selected
      workspace behavior with no console errors or horizontal overflow.
