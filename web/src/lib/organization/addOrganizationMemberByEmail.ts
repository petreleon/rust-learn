import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { OrganizationRequestError } from "./OrganizationRequestError";
import { organizationErrorFromResponse } from "./organizationErrorFromResponse";
import { type AddOrganizationMemberResult } from "./AddOrganizationMemberResult";
import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export async function addOrganizationMemberByEmail({
  apiRoot = "/api",
  email,
  organizationId,
  roleName,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationRequestOptions & { organizationId: number; email: string; roleName?: string }): Promise<AddOrganizationMemberResult> {
  const trimmedToken = token.trim();
  if (!trimmedToken) throw new OrganizationRequestError("Token required.", 401, "missing_token");
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetch(`${apiRoot}/organizations/${organizationId}/members`, {
      body: JSON.stringify({ email: email.trim(), role_name: roleName }),
      headers: { Accept: "application/json, text/plain", Authorization: `Bearer ${trimmedToken}`, "Content-Type": "application/json" },
      method: "POST",
      signal: controller.signal,
    });
    if (!response.ok) throw await organizationErrorFromResponse(response);
    return (await response.json()) as AddOrganizationMemberResult;
  } catch (error) {
    if (error instanceof OrganizationRequestError) throw error;
    if (error instanceof DOMException && error.name === "AbortError") throw new OrganizationRequestError("Timed out.", 0, "timeout");
    throw new OrganizationRequestError(error instanceof Error ? error.message : "Failed.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
