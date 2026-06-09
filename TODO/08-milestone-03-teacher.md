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
- [ ] Teacher-owned course list with scoped permissions and lifecycle status.
- [ ] Course authoring detail payload with chapters, content, publication
      status, ownership, and reward policy summary.
- [ ] Upload URL and media-processing status contract, including queued,
      processing, failed, retried, and complete states.
- [ ] Enrollment request list and decision endpoints with pagination.
- [ ] Student progress endpoint for teacher-visible course learners.
- [ ] Course reward candidate queue and teacher-decision endpoints with
      conflict handling.

## Routes And Screens

- [ ] `/teach` teacher dashboard.
- [x] `/teach/apply` teacher application journey.
  - [x] Product route loads `GET /api/me` and
        `GET /api/teacher-applications/me`, links from `/teach`, preserves
        draft form data in session storage, and never asks applicants to type
        raw organization, course, or application ids.
- [ ] `/teach/courses` owned or permitted courses list.
- [ ] `/teach/courses/[courseId]` course workspace.
- [ ] `/teach/courses/[courseId]/content` course content authoring.
- [ ] `/teach/courses/[courseId]/enrollments` enrollment queue and roster.
- [ ] `/teach/courses/[courseId]/students` student progress.
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

- [ ] Show lifecycle state: draft, submitted, needs changes, approved,
      published, archived, suspended.
- [ ] Allow only permitted actions for create, edit, publish, archive, and
      settings.
- [ ] Use structured chapter/content editing instead of freeform API fields.
- [ ] Show upload URL expiry and retry path.
- [ ] Show worker processing states after video processing is queued.
- [ ] Handle object upload success but content record failure, and content
      record success but processing failure.

## Enrollment And Reward Review

- [ ] Show enrollment requests with learner context, requested status, and
      decision actions.
- [ ] Show roster with access state, progress, and reward eligibility cues.
- [ ] Show reward candidates filtered by status and course.
- [ ] Allow teacher approval/rejection only with course-scoped permission.
- [ ] Keep reward amount review absent from teacher course screens unless the
      same user also has separate platform permission, and then route them to
      platform admin context.
- [ ] On stale candidate state, show conflict and refresh path.

## Teacher Edge Cases

- [ ] Teacher has one course permission but no general teacher role label.
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
- [ ] Tests for approved teacher, applicant-only user, course-scoped teacher,
      denied teacher route, stale reward candidate, and failed upload state.
  - [x] Teacher application tests cover current-user snapshot, duplicate open
        conflict, rejected reapply, idempotency replay, applicant-only submit,
        reviewer permissions, and course-scope approval.
- [ ] Docker Compose E2E path: teacher application or teacher login, course
      workspace, content/upload state, reward candidate decision, log scan.
- [ ] Kubernetes smoke path loads `/teach`, `/teach/apply`, and one course
      workspace state without console errors or horizontal overflow.
