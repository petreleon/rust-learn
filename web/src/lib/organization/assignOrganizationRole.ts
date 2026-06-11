import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { OrganizationRequestError } from "./OrganizationRequestError";
import { organizationErrorFromResponse } from "./organizationErrorFromResponse";
import { type AssignOrganizationRolePayload } from "./AssignOrganizationRolePayload";
import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export async function assignOrganizationRole({
  apiRoot = "/api",
  organizationId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationRequestOptions & { organizationId: number; payload: AssignOrganizationRolePayload }): Promise<string> {
  const trimmedToken = token.trim();
  if (!trimmedToken) throw new OrganizationRequestError("Token required.", 401, "missing_token");
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetch(`${apiRoot}/organizations/${organizationId}/users/${payload.userId}/roles`, {
      body: JSON.stringify({ role_name: payload.roleName }),
      headers: { Accept: "application/json, text/plain", Authorization: `Bearer ${trimmedToken}`, "Content-Type": "application/json" },
      method: "POST",
      signal: controller.signal,
    });
    if (!response.ok) throw await organizationErrorFromResponse(response);
    return await response.text();
  } catch (error) {
    if (error instanceof OrganizationRequestError) throw error;
    if (error instanceof DOMException && error.name === "AbortError") throw new OrganizationRequestError("Timed out.", 0, "timeout");
    throw new OrganizationRequestError(error instanceof Error ? error.message : "Failed.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
