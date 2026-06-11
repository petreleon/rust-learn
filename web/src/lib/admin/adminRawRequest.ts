import { AdminRequestError } from "./AdminRequestError";
import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { rawRequest } from "./rawRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";

export async function adminRawRequest({
  accept = "application/json, text/plain",
  apiRoot = "/api",
  body,
  method = "GET",
  path,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & {
  accept?: string;
  body?: string;
  method?: string;
  path: string;
}): Promise<Response> {
  const trimmedToken = token?.trim() || "";
  if (!trimmedToken) {
    throw new AdminRequestError("A sign-in token is required.", 401, "missing_token");
  }

  return rawRequest({
    accept,
    apiRoot,
    body,
    headers: {
      Authorization: `Bearer ${trimmedToken}`,
    },
    method,
    path,
    timeoutMs,
  });
}
