import { AdminRequestError } from "./AdminRequestError";
import { codeFromStatus } from "./codeFromStatus";
import { type AdminErrorEnvelope } from "./AdminErrorEnvelope";

export async function adminErrorFromResponse(response: Response) {
  const fallbackCode = codeFromStatus(response.status);
  const contentType = response.headers.get("content-type") || "";

  if (contentType.includes("application/json")) {
    const body = (await response.json()) as AdminErrorEnvelope;
    return new AdminRequestError(
      body.error?.message || response.statusText || "Admin request failed.",
      response.status,
      body.error?.code || fallbackCode,
    );
  }

  const text = await response.text();
  return new AdminRequestError(text || response.statusText || "Admin request failed.", response.status, fallbackCode);
}
