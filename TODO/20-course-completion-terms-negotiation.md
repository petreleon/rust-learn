# TODO 20: Course Completion Terms Negotiation

Created: 2026-06-16.
Status: implemented and checked.

Objective: each course now has its own negotiated completion terms between the
teacher and the course sponsor/counterparty:

- reward paid per student course completion
- maximum active enrolled students for the course
- versioned negotiation lifecycle
- audit trail
- accepted terms hand off to existing course-scoped reward policy machinery

TODO/18 backend architecture and TODO/19 web architecture were not rewritten.
This work follows the existing Level 2 boundaries.

## Implemented Backend Shape

Domain:

- `src/domain/learning/course/terms.rs`
- Pure course terms vocabulary for statuses, audit event types, parsing, and
  transition checks.

Application:

- `src/application/learning/manage_course_completion_terms/`
- Use-case boundary for list, submit, counter, accept, reject, and withdraw.
- Store/port traits keep Diesel, Actix, and HTTP DTOs outside the use-case core.
- Application validation rejects invalid reward amounts and invalid capacity.

Infra/Postgres:

- `migrations/2026-06-16-230000_create_course_completion_terms/`
- `course_completion_terms`
- `course_completion_term_audit_events`
- Diesel models and schema regenerated through Make/Compose.
- Store lives under `src/infra/postgres/learning`, because the aggregate owns
  both course capacity and reward policy handoff.
- Accepted terms create/activate a course-scoped `course_completion`
  `reward_policies` row and supersede older active course terms.

HTTP:

- `GET /api/courses/teaching/{id}/completion-terms`
- `POST /api/courses/teaching/{id}/completion-terms/proposals`
- `PUT /api/courses/teaching/{id}/completion-terms/{terms_id}/counter`
- `PUT /api/courses/teaching/{id}/completion-terms/{terms_id}/accept`
- `PUT /api/courses/teaching/{id}/completion-terms/{terms_id}/reject`
- `PUT /api/courses/teaching/{id}/completion-terms/{terms_id}/withdraw`
- Route handlers stay thin and map errors at the HTTP boundary.

Enrollment:

- Course enrollment approval checks active course completion terms before
  granting access.
- Full courses return `course_capacity_full` instead of allowing direct API
  bypass.

## Implemented Web Shape

Teacher course workspace:

- `web/src/features/teacher/course-completion-terms/`
- Feature-owned API adapter, model types, route state, and components.
- The course workspace mounts the terms route with session-aware permission
  gates.
- Teachers can propose/withdraw.
- Sponsor/admin-capable users can counter, accept, or reject from the course
  workspace when their session permissions allow it.
- Audit history and active/open terms are visible in the workspace.

Remaining product polish:

- A dedicated organization/admin negotiation inbox can still be added later.
- A waitlist-specific UX can still be added if product chooses waitlisting
  instead of hard rejection for full courses.
- Budget reservation, expiry windows, legal text, or multi-party signatures were
  intentionally not added.

## Checked

- `cargo fmt --all`
- `cargo check --lib`
- `make migrate && make schema`
- `LIBRARY_PATH=/opt/homebrew/Cellar/libpq/18.4/lib DYLD_FALLBACK_LIBRARY_PATH=/opt/homebrew/Cellar/libpq/18.4/lib LDFLAGS='-L/opt/homebrew/Cellar/libpq/18.4/lib' cargo test manage_course_completion_terms`
- `make test-compose CARGO_TEST_ARGS='--lib manage_course_completion_terms'`
- `cd web && npm run architecture:scan:strict`
- `make web-lint`
- `cd web && npm run test -- src/features/teacher/course-completion-terms/__tests__/CourseCompletionTermsRoute.test.tsx src/features/teacher/course-workspace/__tests__/TeacherCourseWorkspaceRoute.test.tsx`
- `make web-build`

Compose note: a broader `make test-compose CARGO_TEST_ARGS=manage_course_completion_terms`
was stopped because Cargo was compiling unrelated integration test binaries
before applying the filter. The focused Compose check above verifies the new
library use-case tests through the project Make/Compose path.

## Self Critique Result

The final shape avoids the three wrong directions:

- Not stored directly on `courses`, because terms need lifecycle, history, and
  immutable accepted versions.
- Not folded into `reward_policies`, because capacity belongs to learning and
  enrollment, while payout still belongs to rewards after acceptance.
- Not frontend-only, because enrollment capacity and reward policy handoff must
  be enforced server-side.

The remaining compromise is UI placement: counterparty actions are available in
the teacher course workspace through permissions, but a dedicated sponsor review
queue is still a future product workflow rather than part of this delivery.
