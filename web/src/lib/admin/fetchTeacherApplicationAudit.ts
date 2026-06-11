import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type TeacherApplicationAuditEvent } from "./TeacherApplicationAuditEvent";

export async function fetchTeacherApplicationAudit({
  apiRoot = "/api",
  applicationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { applicationId: number }): Promise<TeacherApplicationAuditEvent[]> {
  return adminJsonRequest({
    apiRoot,
    path: `/teacher-applications/${applicationId}/audit`,
    timeoutMs,
    token,
  });
}
