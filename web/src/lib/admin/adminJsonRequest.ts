import { adminRawRequest } from "./adminRawRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";

export async function adminJsonRequest<T>({
  accept = "application/json, text/plain",
  apiRoot,
  body,
  method,
  path,
  timeoutMs,
  token,
}: AdminRequestOptions & {
  accept?: string;
  body?: string;
  method?: string;
  path: string;
}): Promise<T> {
  const response = await adminRawRequest({ accept, apiRoot, body, method, path, timeoutMs, token });
  return (await response.json()) as T;
}
