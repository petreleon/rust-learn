import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { OrganizationRequestError } from "./OrganizationRequestError";
import { organizationErrorFromResponse } from "./organizationErrorFromResponse";
import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export async function organizationRawRequest({
  accept = "application/json, text/plain",
  apiRoot = "/api",
  method = "GET",
  path,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationRequestOptions & {
  accept?: string;
  method?: string;
  path: string;
}): Promise<Response> {
  const trimmedToken = token.trim();
  if (!trimmedToken) {
    throw new OrganizationRequestError("A sign-in token is required.", 401, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}${path}`, {
      headers: {
        Accept: accept,
        Authorization: `Bearer ${trimmedToken}`,
      },
      method,
      signal: controller.signal,
    });

    if (!response.ok) {
      throw await organizationErrorFromResponse(response);
    }

    return response;
  } catch (error) {
    if (error instanceof OrganizationRequestError) {
      throw error;
    }

    if (error instanceof DOMException && error.name === "AbortError") {
      throw new OrganizationRequestError("Organization request timed out.", 0, "timeout");
    }

    throw new OrganizationRequestError("Organization request failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
