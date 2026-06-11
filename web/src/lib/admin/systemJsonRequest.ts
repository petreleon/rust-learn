import { rawRequest } from "./rawRequest";

export async function systemJsonRequest<T>({
  acceptedStatuses = [],
  apiRoot,
  path,
  timeoutMs,
}: {
  acceptedStatuses?: number[];
  apiRoot: string;
  path: string;
  timeoutMs: number;
}): Promise<T> {
  const response = await rawRequest({
    accept: "application/json, text/plain",
    acceptedStatuses,
    apiRoot,
    method: "GET",
    path,
    timeoutMs,
  });
  return (await response.json()) as T;
}
