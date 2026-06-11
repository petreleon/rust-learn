import { learnerErrorFromResponse } from "./learnerErrorFromResponse";
import { learnerRawRequest } from "./learnerRawRequest";

export async function learnerJsonRequest<T>({
  body,
  method = "GET",
  timeoutMs,
  token,
  url,
}: {
  body?: string;
  method?: string;
  timeoutMs: number;
  token: string;
  url: string;
}): Promise<T> {
  const response = await learnerRawRequest({ body, method, timeoutMs, token, url });
  if (!response.ok) {
    throw await learnerErrorFromResponse(response, "Learner request failed.");
  }

  return (await response.json()) as T;
}
