import { AdminRequestError } from "./AdminRequestError";
import { adminErrorFromResponse } from "./adminErrorFromResponse";

export async function rawRequest({
  accept,
  acceptedStatuses = [],
  apiRoot,
  body,
  headers = {},
  method,
  path,
  timeoutMs,
}: {
  accept: string;
  acceptedStatuses?: number[];
  apiRoot: string;
  body?: string;
  headers?: Record<string, string>;
  method: string;
  path: string;
  timeoutMs: number;
}): Promise<Response> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}${path}`, {
      body,
      headers: {
        Accept: accept,
        ...(body ? { "Content-Type": "application/json" } : {}),
        ...headers,
      },
      method,
      signal: controller.signal,
    });

    if (!response.ok && !acceptedStatuses.includes(response.status)) {
      throw await adminErrorFromResponse(response);
    }

    return response;
  } catch (error) {
    if (error instanceof AdminRequestError) {
      throw error;
    }

    if (error instanceof DOMException && error.name === "AbortError") {
      throw new AdminRequestError("Admin request timed out.", 0, "timeout");
    }

    throw new AdminRequestError("Admin request failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
