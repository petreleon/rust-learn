import { DEFAULT_TIMEOUT_MS } from "./session/DEFAULT_TIMEOUT_MS";

export type KycSubmission = {
  id: number;
  status: string;
  legal_name: string;
  country_code: string;
  document_type: string;
  document_last4: string | null;
  evidence_reference: string | null;
  provider_reference: string | null;
  rejection_reason: string | null;
};

export type KycStatusResponse = {
  submission: KycSubmission | null;
  user_kyc_verified: boolean;
  next_action: "submit" | "wait_for_review" | "resubmit" | "verified" | string;
};

export type SubmitKycPayload = {
  legal_name: string;
  country_code: string;
  document_type: string;
  document_last4?: string;
  evidence_reference?: string;
};

type KycRequestOptions = { apiRoot?: string; timeoutMs?: number; token: string };

export class KycRequestError extends Error {
  constructor(message: string, public status: number) {
    super(message);
    this.name = "KycRequestError";
  }
}

async function kycRequest<T>(path: string, options: KycRequestOptions & { body?: unknown; method?: string }): Promise<T> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), options.timeoutMs ?? DEFAULT_TIMEOUT_MS);
  try {
    const response = await fetch(`${options.apiRoot ?? "/api"}${path}`, {
      body: options.body ? JSON.stringify(options.body) : undefined,
      headers: {
        Authorization: `Bearer ${options.token.trim()}`,
        ...(options.body ? { "Content-Type": "application/json" } : {}),
      },
      method: options.method ?? "GET",
      signal: controller.signal,
    });
    if (!response.ok) {
      throw new KycRequestError((await response.text()) || "KYC request failed.", response.status);
    }
    return (await response.json()) as T;
  } catch (error) {
    if (error instanceof KycRequestError) throw error;
    if (error instanceof DOMException && error.name === "AbortError") {
      throw new KycRequestError("KYC request timed out.", 0);
    }
    throw new KycRequestError("KYC request failed.", 0);
  } finally {
    clearTimeout(timeout);
  }
}

export function fetchKycStatus(options: KycRequestOptions) {
  return kycRequest<KycStatusResponse>("/kyc/me", options);
}

export function submitKyc(options: KycRequestOptions & { payload: SubmitKycPayload }) {
  return kycRequest<KycStatusResponse>("/kyc/me", { ...options, body: options.payload, method: "POST" });
}
