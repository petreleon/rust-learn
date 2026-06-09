# Milestone 3: Teacher Experience

Goal: let teachers apply, manage teaching work, author course content, handle
enrollments, and review student reward candidates without exposing platform
admin controls.

## Self-Criticism Before Building

- [ ] If teacher approval can edit reward amount, reject the flow.
- [ ] If course authoring assumes every teacher is platform-level, reject the
      permission model.
- [ ] If file upload does not show expiry, progress, processing status, retry,
      and failure, reject the upload UI.
- [ ] If a teacher must type student, course, application, or reward ids by
      hand, reject the route.
- [ ] If pending queues do not show conflict or stale-state handling, reject
      them.

## Backend Contracts

- [x] Teacher application self-service endpoint or filter for "my application"
      state.
  - [x] `GET /api/teacher-applications/me` returns the current applicant's
        latest application and owned audit trail without requiring review
        permission or raw application ids.
  - [x] Open duplicate applications return a `409` conflict while idempotent
        retry keys still replay safely and rejected applications can be
        followed by a fresh submission.
- [x] Teacher-owned course list with scoped permissions and lifecycle status.
  - [x] `GET /api/courses/teaching` returns only courses where the current
        user has direct, delegated, organization-scoped, or platform-scoped
        teaching permissions. The response includes lifecycle status,
        organization names, content and reward-policy summaries, roster
        pressure, reward queue counts, action permissions, and
        `total/limit/offset/search/lifecycle_status` metadata.
- [x] Course authoring detail payload with chapters, content, publication
      status, ownership, and reward policy summary.
  - [x] `GET /api/courses/teaching/{courseId}` returns the scoped course
        dashboard summary, current teacher roles, publication summary, ordered
        chapters, ordered content items, data-present flags, inherited content
        publication status, display state, and latest processing status/error.
        Per-content publication remains explicitly unsupported until the schema
        stores that state.
- [x] Existing chapter/content mutation endpoints enforce course ownership for
      authoring paths.
  - [x] Chapter/content list, create, update, delete, and upload-url paths now
        verify that the path chapter belongs to the path course, and that
        updated/deleted content belongs to the path chapter before acting.
- [ ] Upload URL and media-processing status contract, including queued,
      processing, failed, retried, and complete states.
- [x] Enrollment request list and decision endpoints with pagination.
  - [x] `GET /api/courses/teaching/{courseId}/enrollments` returns a
        course-scoped enrollment workspace for teachers with enrollment
        permission: course summary, teacher roles, paginated join requests,
        learner names/emails/readiness, roster learners, access state,
        unsupported progress/reward-eligibility flags, and `open`/status/all
        filtering. Existing decision and removal endpoints remain the mutation
        contracts.
- [x] Student progress endpoint for teacher-visible course learners.
  - [x] `GET /api/courses/teaching/{courseId}/students` returns enrolled
        learners for teachers with enrollment or reward visibility permission,
        course/content totals, explicit unsupported lesson-progress fields,
        latest join-request status, and reward-candidate evidence/counts for
        each learner.
- [ ] Course reward candidate queue and teacher-decision endpoints with
      conflict handling.

## Routes And Screens

- [x] `/teach` teacher dashboard.
  - [x] Real product route loads `GET /api/me`,
        `GET /api/courses/teaching`, and
        `GET /api/teacher-applications/me`; it shows application status,
        course health, enrollment pressure, reward review pressure, and
        permission chips without raw id entry or teacher-side reward amount
        controls.
- [x] `/teach/apply` teacher application journey.
  - [x] Product route loads `GET /api/me` and
        `GET /api/teacher-applications/me`, links from `/teach`, preserves
        draft form data in session storage, and never asks applicants to type
        raw organization, course, or application ids.
- [x] `/teach/courses` owned or permitted courses list.
  - [x] Product list supports title search, lifecycle filtering, mobile
        layout, empty state, backend failure state, and scoped action chips.
        Deeper workspace/enrollment/reward routes remain disabled instead of
        linking to unfinished screens.
- [x] `/teach/courses/[courseId]` course workspace.
  - [x] Real product route loads `GET /api/me` and
        `GET /api/courses/teaching/{courseId}`; it shows lifecycle,
        ownership, teacher roles, permissions, content structure, processing
        state, roster pressure, reward review pressure, signed-out state, and
        permission failure state without raw id forms or platform amount-review
        controls.
- [x] `/teach/courses/[courseId]/content` course content authoring.
  - [x] Real product route loads `GET /api/me` and
        `GET /api/courses/teaching/{courseId}`, creates chapters through
        `POST /api/courses/{courseId}/chapters`, creates text/article content
        through `POST /api/courses/{courseId}/chapters/{chapterId}/contents`,
        refreshes the structured outline after success, keeps chapter
        selection human-readable, and disables authoring actions when the
        session lacks course content permission.
- [x] `/teach/courses/[courseId]/enrollments` enrollment queue and roster.
  - [x] Real product route loads `GET /api/me` and
        `GET /api/courses/teaching/{courseId}/enrollments`, links from course
        cards and workspace actions, filters request status, approves,
        waitlists, or rejects join requests through structured controls,
        removes roster access with two-step confirmation, and never asks
        teachers to type learner or request ids.
- [x] `/teach/courses/[courseId]/students` student progress.
  - [x] Real product route loads `GET /api/me` and
        `GET /api/courses/teaching/{courseId}/students`, links from the course
        workspace, shows enrolled learners, content totals, unsupported lesson
        progress, latest enrollment state, and reward evidence without raw
        learner id entry.
- [ ] `/teach/courses/[courseId]/rewards` reward candidate review.

## Teacher Application

- [x] Support new application, submitted, needs changes, approved, rejected,
      and duplicate/idempotency states.
- [x] Preserve draft form state and portfolio links.
- [x] Explain platform, organization, and course scope choices.
- [x] Show sponsor context without requiring raw organization id entry.
  - [x] Self-service organization/course choices use resolved session names
        rather than applicant-entered ids. Organization nomination tracking
        remains open for the organization milestone.
- [x] Show decision reason and audit trail after review.
- [x] Block resubmission or guide changes based on backend status.
  - [x] Submitted/approved states block duplicate form submission,
        needs-changes explains the missing edit/resubmission contract, rejected
        states can reveal a fresh application form, and stale `409` conflicts
        refresh the visible application state.

## Course Authoring

- [x] Show lifecycle state: draft, submitted, needs changes, approved,
      published, archived, suspended.
- [x] Show structured chapter/content outline instead of freeform API fields.
- [ ] Allow only permitted actions for create, edit, publish, archive, and
      settings.
  - [x] Chapter and text/article content creation are permission-aware in the
        content authoring route. Editing, publishing, archiving, destructive
        actions, and settings remain open.
- [x] Use structured chapter/content authoring instead of freeform API fields.
  - [x] `/teach/courses/[courseId]/content` replaces raw course/chapter id
        entry with structured chapter creation and a chapter-title select for
        text/article content creation. Upload/media authoring stays open until
        the upload route is built.
- [ ] Show upload URL expiry and retry path.
- [x] Show worker processing states after video processing is queued.
  - [x] The workspace detail payload and UI surface latest display state,
        processing status, and processing error for content items. Upload
        retry controls remain open until the upload route is built.
- [ ] Handle object upload success but content record failure, and content
      record success but processing failure.

## Enrollment And Reward Review

- [x] Show enrollment requests with learner context, requested status, and
      decision actions.
- [x] Show roster with access state, progress, and reward eligibility cues.
  - [x] Roster cards show learner identity, access state, course roles,
        latest join-request status, and explicit "not tracked yet" progress
        and reward-eligibility cues until the student-progress route exists.
- [ ] Show reward candidates filtered by status and course.
- [ ] Allow teacher approval/rejection only with course-scoped permission.
- [ ] Keep reward amount review absent from teacher course screens unless the
      same user also has separate platform permission, and then route them to
      platform admin context.
- [ ] On stale candidate state, show conflict and refresh path.

## Teacher Edge Cases

- [x] Teacher has one course permission but no general teacher role label.
  - [x] Rust contract coverage proves a course-scoped teacher can load their
        teaching dashboard while an unrelated platform user sees no courses.
- [ ] Teacher has permissions in multiple organizations or courses.
- [ ] Course is archived or suspended after the teacher opens it.
- [ ] Upload URL expires before file upload completes.
- [ ] Worker fails processing after upload succeeded.
- [ ] Enrollment request is already handled by another user.
- [ ] Reward candidate is blocked by fraud policy after it appears in queue.
- [ ] Teacher application changes status while form is open.

## Acceptance Evidence

- [ ] Desktop and mobile rendered checks for teacher dashboard, application,
      course workspace, upload UI, enrollment queue, and reward review.
  - [x] `/teach/apply` evidence: Browser signed-out smoke, mocked Playwright
        desktop submit, stale `409` conflict refresh, needs-changes mobile, and
        rejected mobile reapply with no horizontal overflow.
  - [x] `/teach` and `/teach/courses` evidence: Browser signed-out desktop and
        mobile smoke, mocked Playwright desktop dashboard, mobile course
        search/lifecycle filtering, no-access state, plain-text backend `500`
        state, no internal operations console text for non-admin sessions, no
        console errors, and no horizontal overflow.
  - [x] `/teach/courses/[courseId]` evidence: Browser signed-out desktop and
        mobile smoke, mocked Playwright course-list-to-workspace navigation,
        structured content outline, failed-processing state, scoped permission
        failure state, no internal operations console text, no amount-review
        language, no console errors, and no horizontal overflow.
  - [x] `/teach/courses/[courseId]/content` evidence: Browser signed-out
        desktop and mobile smoke plus mocked Playwright authenticated desktop
        authoring from course workspace to content route, chapter creation,
        text/article content creation, mobile no-edit-permission disabled
        state, hidden internal operations console, no amount-review language,
        no console errors, and no horizontal overflow.
  - [x] `/teach/courses/[courseId]/enrollments` evidence: mocked Playwright
        signed-out route, authenticated course-workspace-to-enrollment
        navigation, join-request decision, roster two-step removal, mobile
        permission-denied state, hidden internal operations console, no
        amount-review language, no console errors, and no horizontal overflow.
  - [x] `/teach/courses/[courseId]/students` evidence: mocked Playwright
        signed-out route, authenticated course-workspace-to-students
        navigation, unsupported progress state, reward evidence display,
        mobile permission-denied state, hidden internal operations console, no
        amount-review language, no console errors, and no horizontal overflow.
- [ ] Tests for approved teacher, applicant-only user, course-scoped teacher,
      denied teacher route, stale reward candidate, and failed upload state.
  - [x] Teacher application tests cover current-user snapshot, duplicate open
        conflict, rejected reapply, idempotency replay, applicant-only submit,
        reviewer permissions, and course-scope approval.
  - [x] Teacher course dashboard tests cover response shape, course-scoped
        permission visibility, lifecycle status, content summary, roster queue,
        reward queue, action permissions, outsider empty result, route
        registration, and frontend helper text-error normalization.
  - [x] Teacher course workspace tests cover scoped detail response shape,
        teacher roles, publication summary, chapter/content structure,
        inherited publication status, content display state, outsider `403`,
        route registration, frontend helper parsing, and frontend helper
        `403`/`404` text-error normalization.
  - [x] Teacher content authoring tests cover frontend helper request bodies,
        helper `403`/invalid-chapter error normalization, chapter creation,
        text/article content creation, and backend cross-course
        chapter/content path guardrails.
  - [x] Teacher enrollment tests cover the paginated enrollment workspace
        contract, default open-request filtering, pending-only filtering,
        learner context, roster access state, unsupported progress/reward
        eligibility flags, permission denial, frontend helper parsing,
        decision request bodies, removal request bodies, and text-error
        normalization.
  - [x] Teacher student-progress tests cover the scoped students endpoint,
        enrolled learner visibility, unsupported lesson-progress flags, content
        totals, reward-candidate counts/latest evidence, outsider `403`,
        frontend helper parsing, and frontend helper text-error normalization.
- [ ] Docker Compose E2E path: teacher application or teacher login, course
      workspace, content/upload state, reward candidate decision, log scan.
- [ ] Kubernetes smoke path loads `/teach`, `/teach/apply`, and one course
      workspace state without console errors or horizontal overflow.
