import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type TeacherApplication } from "./TeacherApplication";
import { type TeacherApplicationDecisionOptions } from "./TeacherApplicationDecisionOptions";

export async function decideTeacherApplication({
  apiRoot = "/api",
  applicationId,
  decisionReason,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherApplicationDecisionOptions): Promise<TeacherApplication> {
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify({
      decision_reason: decisionReason?.trim() || null,
      status,
    }),
    method: "PUT",
    path: `/teacher-applications/${applicationId}/decision`,
    timeoutMs,
    token,
  });
}
