# Backend Contract Gaps

Resolve or explicitly document these backend/frontend contracts before product
screens pretend to be complete.

- [x] Define or add a current-user/session endpoint that returns profile,
      email verification status, platform permissions, organization
      memberships, course enrollments, and delegated permissions for route
      guards.
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
- [ ] Define upload-job or media-processing status APIs before the teacher
      upload UI promises progress or retry visibility.
- [ ] Define searchable user, organization, course, application, reward,
      delegation, and fraud-block lookups so normal users do not need to know
      numeric ids.
- [ ] Normalize or document API error shapes. Until that is done, frontend
      helpers must parse both JSON and text responses safely.
- [ ] Define pagination metadata and filtering contracts for queues, reports,
      audit logs, and dashboards before building reusable table components.
- [ ] Define wallet operation response states for MetaMask-required,
      platform-gas, tax, insufficient funds, pending deposit, confirmed
      deposit, retirement, and reconciliation cases.
