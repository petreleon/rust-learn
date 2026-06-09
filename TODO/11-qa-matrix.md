# QA Matrix

Use this matrix to decide what evidence is required before marking frontend
work complete.

## Rendered Browser Checks

- [ ] Desktop first viewport renders meaningful product content, not the
      operations console unless `/ops` is under test.
- [ ] Mobile first viewport shows page identity, primary status, primary
      action, and navigation entry.
- [ ] No horizontal overflow on common mobile widths: 360, 390, 414.
- [ ] No text overlap, clipped buttons, invisible focus, or layout shift after
      loading completes.
- [ ] Framework overlays are absent.
- [ ] Console has no relevant warnings or errors.
- [ ] Deep links load correctly after refresh.
- [ ] Route transitions preserve or intentionally discard dirty state with
      visible confirmation.

### Recent Route Evidence

- [x] `/organizations/[organizationId]/courses` rendered QA covers populated
      desktop course list, dashboard-to-courses navigation, search/lifecycle/
      reward filter interaction, mobile first viewport/no-overflow, missing
      course-list permission, empty course data, and backend `500` retry state.
      Normal populated, mobile, denied, and empty flows had no relevant console
      errors; the backend failure fixture intentionally produced `500`
      resource console evidence. BrowserMCP was unavailable
      (`Transport closed`), so Playwright supplied screenshots and interaction
      proof.
- [x] `/organizations/[organizationId]/reports` rendered QA covers populated
      desktop report, CSV download/status, dashboard-to-report navigation,
      mobile first viewport/no-overflow, missing report permission, empty
      report data, and backend `500` retry state. Normal populated, mobile,
      denied, and empty flows had no relevant console errors; the backend
      failure fixture intentionally produced `500` resource console evidence.
      BrowserMCP was unavailable (`Transport closed`), so Playwright supplied
      screenshots and interaction proof.

## API Helper Contract Checks

- [ ] JSON success with expected shape.
- [ ] JSON error with structured message.
- [ ] Plain text error from current Actix handlers.
- [ ] CSV success with content disposition and filename.
- [ ] Empty CSV success.
- [ ] `401` unauthenticated.
- [ ] `403` permission denied.
- [ ] `404` missing resource.
- [ ] `409` conflict or invalid state.
- [ ] Timeout.
- [ ] Network failure.
- [ ] Backend `5xx`.
- [ ] Aborted request during route change.

## Permission And Scope Checks

- [ ] No permission beyond learner defaults.
- [ ] Platform permission only.
- [ ] Organization permission only.
- [ ] Course permission only.
- [ ] Delegated permission active.
- [ ] Delegated permission expired.
- [ ] Delegated permission revoked.
- [ ] Delegated permission scoped to another organization or course.
- [ ] Multiple organizations with different permissions.
- [ ] Multiple courses with different permissions.
- [ ] Privileged role label without required permission.
- [ ] Same permission across different role labels.

## State Checks

- [ ] Loading.
- [ ] Empty.
- [ ] Success.
- [ ] Dirty form.
- [ ] Validation error.
- [ ] Permission denied.
- [ ] Session expired.
- [ ] Missing resource.
- [ ] Conflict or stale state.
- [ ] Network failure.
- [ ] Backend failure.
- [ ] Retry success.
- [ ] Retry failure.
- [ ] Destructive action confirmation.
- [ ] Post-action audit/history visible.

## Async Workflow Checks

- [ ] Upload URL requested.
- [ ] Upload URL expired.
- [ ] Upload in progress.
- [ ] Object uploaded but content create fails.
- [ ] Content created but processing queue fails.
- [ ] Processing queued.
- [ ] Processing failed.
- [ ] Processing retried.
- [ ] Processing complete.
- [ ] Reward pending teacher approval.
- [ ] Reward teacher approved.
- [ ] Reward amount approved.
- [ ] Reward token pending.
- [ ] Reward token confirmed.
- [ ] Reward wallet credited.
- [ ] Reward needs reconciliation.
- [ ] Reward failed.
- [ ] Reward blocked by fraud control.

## Docker Compose Evidence

- [ ] Web route served from Compose web container.
- [ ] Browser API requests use Compose proxy/API root correctly.
- [ ] App health and readiness pass.
- [ ] Worker healthcheck passes when worker-related flow is touched.
- [ ] Recent app/web logs have no relevant warning/error lines.
- [ ] Recent worker logs have no relevant warning/error lines when worker flow
      is touched.
- [ ] Database-backed state is used where the flow depends on persistence.
- [ ] RustFS/S3 is used where upload or media flow is touched.
- [ ] Anvil/Ethereum is used where token or wallet flow is touched.

## Kubernetes Evidence

- [ ] Product route served from Kubernetes web deployment.
- [ ] Browser API requests use Kubernetes API root correctly.
- [ ] Deployment rollout is available.
- [ ] Pods are ready.
- [ ] In-cluster web-to-api readiness check passes.
- [ ] Product route smoke text is not only `/healthz`.
- [ ] Recent rust-app logs have no relevant warning/error lines.
- [ ] Recent web logs have no relevant warning/error lines.
- [ ] Recent worker logs have no relevant warning/error lines when worker flow
      is touched.
- [ ] Port-forward or ingress path is tested with desktop and mobile browser
      where practical.
