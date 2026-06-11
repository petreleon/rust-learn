import { organizationRawRequest } from "./organizationRawRequest";
import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export async function organizationJsonRequest<T>({
  accept = "application/json, text/plain",
  apiRoot,
  method,
  path,
  timeoutMs,
  token,
}: OrganizationRequestOptions & {
  accept?: string;
  method?: string;
  path: string;
}): Promise<T> {
  const response = await organizationRawRequest({ accept, apiRoot, method, path, timeoutMs, token });
  return (await response.json()) as T;
}
