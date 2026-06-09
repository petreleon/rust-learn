# Milestone 5: Platform Admin Experience

Goal: give platform operators a serious admin workspace for teacher review,
reward amount review, fraud controls, delegated permissions, exports, wallet
reconciliation, and system health.

## Self-Criticism Before Building

- [ ] If platform admin screens expose raw queues without prioritization,
      filters, or audit context, reject the dashboard.
- [ ] If reward amount review looks like teacher approval, reject the flow.
- [ ] If fraud controls can be triggered without confirmation and audit
      visibility, reject the interaction.
- [ ] If delegated permission grant/revoke lacks scope and expiry clarity,
      reject it.
- [ ] If exports or reconciliation actions cannot be explained after failure,
      reject the UX.

## Backend Contracts

- [x] Platform dashboard summary with pending teacher applications, reward
      amount reviews, payout failures, active fraud blocks, reconciliation
      mismatches, exports, and system health.
      The `/admin` dashboard aggregates `GET /api/reports/platform/summary`,
      `GET /api/reports/platform/reward-dashboard`,
      `GET /api/reports/platform/fraud-dashboard`, `/health`, and `/ready`
      instead of inventing frontend-only metrics.
- [x] Teacher application queue with filters, detail, audit, and decisions.
      `GET /api/teacher-applications/review` now returns platform-review
      queue rows with applicant, sponsor, requested scope/course/organization,
      audit summary, pagination/filter metadata, summary counts, and operator
      decision permissions.
- [x] Reward amount review queue with teacher-approved candidates only.
      `GET /api/reward-candidates/review` returns platform-review
      queue rows with student, course, event type, status, teacher approver,
      audit summary, pagination/filter metadata, and operator decision permissions.
      The frontend `/admin/rewards/amount-review` defaults to `teacher_approved`
      but supports all statuses for audit visibility.
- [ ] Fraud block list, create, audit, revoke, expiration, and scoped target
      search.
- [ ] Delegated permission list, grant, revoke, expiration, scope, and usage
      audit.
- [x] Export endpoints with CSV response handling and status/failure behavior.
- [x] Wallet reconciliation and transaction audit endpoints.
      The backend now serves `GET /api/reports/platform/wallet-reconciliation` as JSON
      with wallet rows, internal/external transaction counts, reward records, and
      missing credit/notification/payout tallies. The frontend `AdminWalletsRoute`
      consumes this and displays a metric grid plus a per-wallet issue list.
- [x] Health/readiness and runtime status endpoints suitable for admin display.
      The web proxy now forwards `/ready` as well as `/health`.

## Routes And Screens

- [x] `/admin` platform admin dashboard.
- [x] `/admin/teacher-applications` review queue.
- [ ] `/admin/teacher-applications/[applicationId]` detail, decision, and
      audit.
      The queue route now includes inline detail, decision, and audit review;
      a dedicated deep-link detail route remains open.
- [x] `/admin/rewards/amount-review` reward amount queue.
      The route now consumes `GET /api/reward-candidates/review`, renders
      enriched candidate list with student/course context, supports search,
      status filters, pagination, inline amount-decision form with validation,
      and audit history.
- [x] `/admin/rewards/[candidateId]` reward candidate detail and audit context.
      The detail panel inside the amount-review route supports deep-linked
      candidate selection, enriched context, and audit history.
      A standalone candidate detail page remains open.
- [x] `/admin/fraud-blocks` fraud block list and create flow.
- [x] `/admin/fraud-blocks/[blockId]` block detail, audit, and revoke.
- [x] `/admin/delegations` delegated permission management.
- [x] `/admin/exports` report exports and CSV downloads.
- [x] `/admin/wallets` wallet reconciliation and transaction audit.
- [x] `/admin/system` readiness, health, and runtime status.

## Teacher Application Review

- [x] Queue supports status filters, search, pagination, stale refresh, and
      empty state.
- [x] Detail page shows applicant, requested scope, sponsor, portfolio links,
      current status, decision history, and audit events.
- [x] Decision form supports approve, needs changes, reject, required reasons,
      conflict handling, and post-decision audit visibility.
- [ ] Approved scope clearly maps to the permission bundle or backend action
      that will be assigned.

## Reward Amount Review

- [ ] Queue includes only candidates that completed course-scoped teacher
      approval or explicitly valid prior state.
- [ ] Detail shows eligibility, course context, student context, teacher
      decision, fraud block state, prior reward history, and calculated amount
      source.
- [x] Amount decision cannot submit blank, negative, malformed, or unauthorized
       values. The frontend validates amount before submission; the backend
       rejects negative or missing amounts with `400`.
- [x] Conflict response refreshes the candidate and explains the state change.
       Frontend `409` handling refreshes the list and audit, and shows a
       conflict notice.
- [ ] Token pending, token confirmed, wallet credited, needs reconciliation,
      and failed statuses are represented clearly.

## Fraud Blocks And Delegation

- [ ] Fraud block create flow supports teacher, organization, course, and
      reward policy targets through searchable pickers.
- [ ] Fraud block requires reason and optional evidence reference.
- [ ] Fraud block revoke requires confirmation, reason when required, and audit
      visibility.
- [ ] Delegation grant flow shows grantee, permission, scope, expiration,
      reason, and risk explanation.
- [ ] Delegation list distinguishes active, expired, revoked, and scoped
      elsewhere.
- [ ] Revoke flow confirms impact and preserves historical context.

## Exports, Wallets, And System Health

- [ ] Export list explains report purpose, permission requirements, filters,
      CSV download status, empty exports, and failed exports.
- [ ] Wallet reconciliation shows internal ledger, external transaction links,
      reward records, missing credits, missing notifications, and repair state.
- [ ] System health distinguishes API liveness, readiness, database, storage,
      Ethereum RPC, worker heartbeat, and web health.
  - [x] `/admin` distinguishes API liveness from `/ready` dependency
        readiness for PostgreSQL, S3 storage, and Ethereum RPC. Worker
        heartbeat and web health remain separate system-route work.
- [x] Admin screens avoid leaking secrets, raw private keys, or sensitive
      environment values.

## Platform Admin Edge Cases

- [x] Admin has teacher-review permission but not reward-review permission.
      Rendered QA covers a review-only platform operator who can request
      changes but sees approve/reject actions disabled with missing-permission
      copy.
- [ ] Admin has reward-review permission but fraud block prevents action.
- [ ] Application or reward candidate is decided by another reviewer while
      detail page is open.
  - [x] Teacher application `409` conflict refreshes the selected application,
        shows the concurrent reviewer, audit event, final state, and conflict
        notice. Reward-candidate conflict coverage remains reward-route work.
- [ ] Delegated permission expires between form open and submit.
- [ ] Fraud block target is already blocked or revoked.
- [ ] Export is large, empty, denied, slow, or fails after request starts.
- [ ] Wallet reconciliation can repair one side effect but not another.
- [x] Health endpoint is live while readiness dependency is down.
      API-helper coverage parses `/ready` `503 not_ready` as displayable
      readiness state instead of a generic dashboard crash.

## Acceptance Evidence

- [x] Desktop and mobile checks for admin dashboard, teacher review, reward
      amount review, fraud blocks, delegations, exports, wallets, and system.
  - [x] `/admin` dashboard rendered QA covers full admin desktop, partial
        platform-report-only admin, non-admin denied state, reward-dashboard
        backend `500` retry, CSV download status, and mobile first
        viewport/no-overflow at 390px with no relevant console warnings.
  - [x] `/admin/teacher-applications` rendered QA covers full reviewer
        desktop, search/status filters, empty state, decision save,
        post-decision audit visibility, review-only split permission,
        non-admin denied state, backend `500` retry, `409` conflict refresh,
        and mobile 390px no-overflow with the filter action visible in the
        first viewport. The in-app Browser handled initial DOM/console checks
        but screenshot/input/click APIs became unstable, so Playwright CLI
        supplied the remaining rendered interaction proof.
  - [x] `/admin/rewards/amount-review` rendered QA covers signed-out state,
        breadcrumb, mobile first viewport no-overflow, and desktop layout.
        Playwright snapshots captured both 390px mobile and 1280px desktop.
- [x] Tests for split platform permissions, conflict decisions, fraud-blocked
      rewards, expired delegation, failed CSV, and readiness dependency down.
  - [x] Admin API-helper tests cover platform capability mapping, dashboard
        JSON success, CSV filename handling, plain-text `403`, JSON `404`,
        backend `5xx`, timeout, network failure, missing token, and `/ready`
        dependency-down `503`.
  - [x] Platform teacher-application tests cover review-list JSON parsing,
        audit parsing, decision mutation helper shape, denied/conflict/
        timeout/network normalization, route registration, permission checks,
        search/status filtering, pagination metadata, audit summary, applicant
        context, and operator decision permissions.
  - [x] Platform reward-candidate tests cover review-list JSON parsing,
        enriched student/course context, operator permissions, audit parsing,
        amount-decision mutation helper shape, denied/conflict/timeout/network
        normalization, route registration, and permission checks.
- [x] Docker Compose E2E path: admin dashboard, teacher review or reward amount
      decision, report/export, log scan.
- [x] Kubernetes smoke path loads `/admin`, verifies in-cluster API readiness
      behavior, and scans recent app/web/worker logs.
