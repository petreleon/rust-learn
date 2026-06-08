# Self-Criticism Protocol

Use this file before turning any TODO item into code. The goal is to keep the
frontend work from drifting back into a prettier operations console.

## Required Critique Passes

Run these passes in order for every milestone, route, or component bundle.
For whole-folder rereads, use
[12-recursive-review.md](12-recursive-review.md) after this local critique.

- [ ] Pass 1: Product honesty. Ask whether the work helps a learner, teacher,
      organization operator, or platform admin complete a real task. If the
      answer is "it mainly helps us poke the API," move it to `/ops` or revise
      the task.
- [ ] Pass 2: Persona and permission honesty. Identify the persona shown in the
      UI, then list the exact platform, organization, course, or delegated
      permissions that control every action. Do not use persona as authority.
- [ ] Pass 3: Data contract honesty. List the backend endpoints, response
      shapes, pagination, filters, error formats, and async statuses needed.
      If any are missing, add a contract task before building fake behavior.
- [ ] Pass 4: State honesty. For the route or component, name the loading,
      empty, success, dirty, denied, expired-session, conflict, network-failure,
      backend-failure, and retry states.
- [ ] Pass 5: Mobile honesty. Check the first mobile viewport before desktop
      refinement. Navigation, primary action, status, and next step must be
      visible without a fragile scroll hunt.
- [ ] Pass 6: Runtime honesty. Define how the change will be proven in the
      browser, through Docker Compose, and through Kubernetes when deployed.
- [ ] Pass 7: Maintenance honesty. Check whether the new code duplicates API
      helpers, permission logic, form behavior, table behavior, or status copy.
      Extract only when reuse is real.

## Agreement Criteria

Only accept a TODO item as implementation-ready when all of these are true.

- [ ] The task has a named user, a route or component surface, and a clear
      next action for that user.
- [ ] The task says what backend data is required and what happens if that data
      is unavailable.
- [ ] The task names permission and scope rules separately from persona labels.
- [ ] The task names at least one edge case that would otherwise be easy to
      miss.
- [ ] The task has desktop, mobile, API-helper, Docker Compose, and Kubernetes
      evidence requirements appropriate to its blast radius.
- [ ] The task does not require normal users to paste JWTs, type raw internal
      ids, or manually toggle permission checkboxes.
- [ ] The task has a route-to-contract-to-test traceability block when it adds
      or changes a product route.

## Rejection Patterns

Reject or rewrite a frontend task when it contains any of these smells.

- [ ] "Build dashboard" without saying which decisions the dashboard supports.
- [ ] "Use role" without also naming resolved permissions and scopes.
- [ ] "Show list" without pagination, empty state, denied state, and stale data
      behavior.
- [ ] "Upload file" without progress, cancellation, retry, expiry, processing
      status, and failed-worker behavior.
- [ ] "Download CSV" without denied, empty, large, slow, and failed download
      states.
- [ ] "Approve" or "revoke" without confirmation, conflict handling, and audit
      visibility.
- [ ] "Kubernetes smoke test" that only checks `/healthz` and never loads the
      product route.
- [ ] "Frontend complete" while the operations console is still the main user
      experience.

## Decision Log Template

Use this small log inside PR descriptions or local notes when a task is fuzzy.

```text
Task:
Persona:
Route or component:
Primary action:
Required permissions and scopes:
Backend contracts:
Known missing contracts:
States covered:
States deferred:
Desktop proof:
Mobile proof:
Compose proof:
Kubernetes proof:
Self-criticism result:
```
