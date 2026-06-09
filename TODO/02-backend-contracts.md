# Backend Contract Gaps

Resolve or explicitly document these backend/frontend contracts before product
screens pretend to be complete.

- [x] Define or add a current-user/session endpoint that returns profile,
      email verification status, platform permissions, organization
      memberships, course enrollments, and delegated permissions for route
      guards.
  - [x] Organization workspace routes now consume the existing organization
        membership scopes from `GET /api/me` for selector cards, dashboard
        permissions, role-only visibility, delegated access, and stale
        workspace handling. Organization health, members, reports, wallet, and
        nomination rows still require dedicated contracts.
- [x] Define course discovery/detail payloads rich enough for product pages:
      description, owner organization, lifecycle status, enrollment state,
      syllabus/chapter summary, teacher metadata, media availability, and reward
      policy summary.
  - [x] `GET /api/courses/catalog` and
        `GET /api/courses/catalog/{courseId}` now return learner-visible course
        summaries/detail, organizations, teachers, content/chapter summaries,
        active course reward policy summary, enrollment state, request ability,
        and access flags.
  - [x] `GET /api/courses/catalog/{courseId}/learn` now returns the
        permission-scoped learner lesson contract: course summary, ordered
        chapters, ordered content items, active first content, upload/media
        display state, processing status/error, and `progress_supported`.
  - [ ] Course `description`, `topics`, and `prerequisites` are still
        nullable/empty because the current course schema does not store them.
        Add schema fields before treating those as authored course content.
- [ ] Define learner course-progress and content-completion endpoints before
      building the learner dashboard as more than static cards.
  - [x] The `/learn` frontend dashboard currently composes existing learner
        catalog, reward-history, wallet, and session contracts; it labels
        progress as not tracked instead of inventing completion state.
  - [x] The lesson-viewer contract explicitly reports
        `progress_supported=false` so the frontend does not pretend persisted
        progress exists.
- [ ] Define assessment APIs before marking assessment-taking screens complete.
- [ ] Define notification list/read APIs before making notification UX a core
      navigation feature.
- [x] Define a teacher application self-service contract for current-user
      application state.
  - [x] `GET /api/teacher-applications/me` now returns the authenticated
        applicant's latest application plus owned audit events, or
        `{ application: null, audit_events: [] }` when no application exists.
  - [x] Duplicate open teacher application submissions now return a `409`
        conflict while same-key idempotent retries still return the existing
        application, and rejected applications can be followed by a new
        submission.
- [x] Define a teacher-owned course dashboard/list contract with scoped
      permissions and lifecycle state.
  - [x] `GET /api/courses/teaching` returns teaching-visible courses with
        `total/limit/offset/search/lifecycle_status`, organization labels,
        content summaries, reward-policy summaries, roster counts, reward
        queue counts, and action permission booleans. Candidate lookup starts
        from direct course roles, organization scope, active delegations, and
        platform scope before per-course permission summaries are built.
- [x] Define a teacher course workspace/detail contract for structured
      authoring context.
  - [x] `GET /api/courses/teaching/{courseId}` returns the scoped course
        summary, teacher roles, ownership labels, publication summary, ordered
        chapters, ordered content items, data-present flags, inherited content
        publication status, display state, and latest processing status/error.
        Per-content publication status is explicitly unsupported until the
        course content schema stores it.
- [x] Harden existing chapter/content authoring endpoints so frontend routes
      can trust course-scoped paths.
  - [x] `GET`/`POST` content, content update/delete, and upload-url handlers
        now verify that path chapters belong to path courses and that path
        content belongs to path chapters before returning or mutating data.
- [x] Define a teacher enrollment workspace contract for join-request and
      roster management.
  - [x] `GET /api/courses/teaching/{courseId}/enrollments` returns only to
        users with course-scoped enrollment permission and includes course
        summary, teacher roles, paginated/filterable join requests, requester
        and reviewer summaries, roster learners, access state, and explicit
        unsupported progress/reward-eligibility flags.
- [x] Define a teacher-visible student progress/evidence contract without
      inventing lesson completion data.
  - [x] `GET /api/courses/teaching/{courseId}/students` returns enrolled
        learners to teachers with enrollment or reward visibility permission,
        content totals, explicit unsupported progress fields, latest enrollment
        state, and real reward-candidate evidence/counts per learner.
- [x] Define course-scoped teacher reward candidate list and decision
      contracts before marking reward review complete.
  - [x] `GET /api/courses/{courseId}/reward-candidates` supports course and
        status-filtered candidate lists, while
        `PUT /api/courses/{courseId}/reward-candidates/{candidateId}/teacher-decision`
        records only teacher approval/rejection decisions. Stale candidates
        return a text `409` that the frontend normalizes and refreshes from.
- [x] Define an organization-scoped course list contract before marking
      organization course visibility complete.
  - [x] `GET /api/organizations/{organizationId}/courses` returns to platform
        or organization-scoped `VIEW_ORGANIZATION` users and includes
        `total/limit/offset/search/lifecycle_status/reward_available`,
        organization labels, course lifecycle, teacher labels, content
        summaries, roster counts, reward-policy summaries, reward queue counts,
        and operator permission booleans. Course editing, publishing,
        organization ownership changes, and reward-policy authoring remain
        separate contracts.
- [x] Define an organization-scoped member directory contract before marking
      member visibility complete.
  - [x] `GET /api/organizations/{organizationId}/members` returns to platform
        or organization-scoped `VIEW_ORGANIZATION` users and includes
        `total/limit/offset/search/role/permission`, organization labels,
        member names/emails, email/KYC readiness, role labels, direct
        permissions, delegated organization permissions, effective
        permissions, and operator action booleans. Invite, role mutation,
        removal, and audit-history behavior remain separate contracts.
- [x] Define an organization-scoped teacher application tracking contract
      before marking sponsored-application visibility complete.
  - [x] `GET /api/organizations/{organizationId}/teacher-applications`
        returns to platform or organization-scoped
        `VIEW_ORG_TEACHER_APPLICATIONS` or
        `NOMINATE_TEACHER_FOR_PLATFORM_REVIEW` users and includes
        `total/limit/offset/search/status`, organization labels, applicant
        names/emails, requested scope, requested organization/course labels,
        sponsor/requested-here booleans, portfolio links, status, reviewer,
        decision reason, audit summary, dashboard summary counts, and operator
        action booleans. A searchable applicant lookup remains open before
        nomination submission should become a normal product form.
- [ ] Define upload-job or media-processing status APIs before the teacher
      upload UI promises progress or retry visibility.
- [ ] Define searchable user, organization, course, application, reward,
      delegation, and fraud-block lookups so normal users do not need to know
      numeric ids.
- [ ] Normalize or document API error shapes. Until that is done, frontend
      helpers must parse both JSON and text responses safely.
- [ ] Define pagination metadata and filtering contracts for queues, reports,
      audit logs, and dashboards before building reusable table components.
  - [x] Teacher course dashboard/list now has pagination and title/lifecycle
        filtering metadata. Enrollment, reward, report, audit, and reusable
        table contracts remain open.
  - [x] Organization reward reports have an existing all-time JSON and CSV
        contract at
        `GET /api/reports/organizations/{organizationId}/reward-dashboard` and
        `.csv`; the frontend now labels it as all-time instead of pretending
        date filters, pagination, payout failures, or reconciliation rows exist.
- [ ] Define wallet operation response states for MetaMask-required,
      platform-gas, tax, insufficient funds, pending deposit, confirmed
      deposit, retirement, and reconciliation cases.
