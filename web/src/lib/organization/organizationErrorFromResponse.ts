import { OrganizationRequestError } from "./OrganizationRequestError";
import { codeFromStatus } from "./codeFromStatus";
import { type OrganizationErrorEnvelope } from "./OrganizationErrorEnvelope";

export async function organizationErrorFromResponse(response: Response) {
  const fallbackCode = codeFromStatus(response.status);
  const contentType = response.headers.get("content-type") || "";

  if (contentType.includes("application/json")) {
    const body = (await response.json()) as OrganizationErrorEnvelope;
    return new OrganizationRequestError(
      body.error?.message || response.statusText || "Organization request failed.",
      response.status,
      body.error?.code || fallbackCode,
    );
  }

  const text = await response.text();
  return new OrganizationRequestError(
    text || response.statusText || "Organization request failed.",
    response.status,
    fallbackCode,
  );
}
