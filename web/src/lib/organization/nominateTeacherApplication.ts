import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { OrganizationRequestError } from "./OrganizationRequestError";
import { organizationErrorFromResponse } from "./organizationErrorFromResponse";
import { type NominateTeacherPayload } from "./NominateTeacherPayload";
import { type NominateTeacherResult } from "./NominateTeacherResult";
import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export async function nominateTeacherApplication({
  apiRoot = "/api",
  organizationId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationRequestOptions & { organizationId: number; payload: NominateTeacherPayload }): Promise<NominateTeacherResult> {
  const body: Record<string, unknown> = {
    applicant_user_id: payload.applicantUserId,
    experience_summary: payload.experienceSummary,
  };
  if (payload.requestedScope) body.requested_scope = payload.requestedScope;
  if (typeof payload.requestedCourseId === "number") body.requested_course_id = payload.requestedCourseId;
  if (payload.portfolioLinks?.length) body.portfolio_links = payload.portfolioLinks;
  if (payload.idempotencyKey) body.idempotency_key = payload.idempotencyKey;

  const trimmedToken = token.trim();
  if (!trimmedToken) {
    throw new OrganizationRequestError("A sign-in token is required.", 401, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}/organizations/${organizationId}/teacher-applications`, {
      body: JSON.stringify(body),
      headers: {
        Accept: "application/json, text/plain",
        Authorization: `Bearer ${trimmedToken}`,
        "Content-Type": "application/json",
      },
      method: "POST",
      signal: controller.signal,
    });

    if (!response.ok) {
      throw await organizationErrorFromResponse(response);
    }

    return (await response.json()) as NominateTeacherResult;
  } catch (error) {
    if (error instanceof OrganizationRequestError) throw error;
    if (error instanceof DOMException && error.name === "AbortError") {
      throw new OrganizationRequestError("Organization request timed out.", 0, "timeout");
    }
    throw new OrganizationRequestError(
      error instanceof Error ? error.message : "Nomination failed.",
      0,
      "network_error",
    );
  } finally {
    clearTimeout(timeout);
  }
}
