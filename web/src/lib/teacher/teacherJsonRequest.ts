import { teacherErrorFromResponse } from "./teacherErrorFromResponse";
import { teacherRawRequest } from "./teacherRawRequest";

export async function teacherJsonRequest<T>({
  body,
  method,
  timeoutMs,
  token,
  url,
}: {
  body?: string;
  method: string;
  timeoutMs: number;
  token: string;
  url: string;
}): Promise<T> {
  const response = await teacherRawRequest({ body, method, timeoutMs, token, url });
  if (!response.ok) {
    throw await teacherErrorFromResponse(response, "Teacher request failed.");
  }

  return (await response.json()) as T;
}
