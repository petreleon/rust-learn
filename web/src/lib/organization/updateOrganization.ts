import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { OrganizationRequestError } from "./OrganizationRequestError";
import { organizationErrorFromResponse } from "./organizationErrorFromResponse";
import { type OrganizationDetail } from "./OrganizationDetail";
import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";
import { type UpdateOrganizationPayload } from "./UpdateOrganizationPayload";

export async function updateOrganization({
  apiRoot = "/api",
  organizationId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationRequestOptions & { organizationId: number; payload: UpdateOrganizationPayload }): Promise<OrganizationDetail> {
  const trimmedToken = token.trim();
  if (!trimmedToken) {
    throw new OrganizationRequestError("A sign-in token is required.", 401, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}/organizations/${organizationId}`, {
      body: JSON.stringify(payload),
      headers: {
        Accept: "application/json, text/plain",
        Authorization: `Bearer ${trimmedToken}`,
        "Content-Type": "application/json",
      },
      method: "PUT",
      signal: controller.signal,
    });

    if (!response.ok) {
      throw await organizationErrorFromResponse(response);
    }

    return (await response.json()) as OrganizationDetail;
  } catch (error) {
    if (error instanceof OrganizationRequestError) {
      throw error;
    }

    if (error instanceof DOMException && error.name === "AbortError") {
      throw new OrganizationRequestError("Organization request timed out.", 0, "timeout");
    }

    throw new OrganizationRequestError(
      error instanceof Error ? error.message : "Organization update failed.",
      0,
      "network_error",
    );
  } finally {
    clearTimeout(timeout);
  }
}
