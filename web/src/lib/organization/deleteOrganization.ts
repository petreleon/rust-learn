import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { OrganizationRequestError } from "./OrganizationRequestError";
import { organizationErrorFromResponse } from "./organizationErrorFromResponse";
import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export async function deleteOrganization({
  apiRoot = "/api",
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationRequestOptions & { organizationId: number }): Promise<void> {
  const trimmedToken = token.trim();
  if (!trimmedToken) throw new OrganizationRequestError("Token required.", 401, "missing_token");
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetch(`${apiRoot}/organizations/${organizationId}`, {
      headers: { Accept: "application/json, text/plain", Authorization: `Bearer ${trimmedToken}` },
      method: "DELETE",
      signal: controller.signal,
    });
    if (!response.ok) throw await organizationErrorFromResponse(response);
  } catch (error) {
    if (error instanceof OrganizationRequestError) throw error;
    if (error instanceof DOMException && error.name === "AbortError") throw new OrganizationRequestError("Timed out.", 0, "timeout");
    throw new OrganizationRequestError(error instanceof Error ? error.message : "Failed.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
