# Backend Contract Gaps

Resolve or explicitly document these backend/frontend contracts before product
screens pretend to be complete.

- [ ] Define or add a current-user/session endpoint that returns profile,
      email verification status, platform permissions, organization
      memberships, course enrollments, and delegated permissions for route
      guards.
- [ ] Define course discovery/detail payloads rich enough for product pages:
      description, owner organization, lifecycle status, enrollment state,
      syllabus/chapter summary, teacher metadata, media availability, and reward
      policy summary.
- [ ] Define learner course-progress and content-completion endpoints before
      building the learner dashboard as more than static cards.
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

