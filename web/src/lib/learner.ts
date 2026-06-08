export type RewardHistoryEntry = {
  approved_amount: string | null;
  course_id: number;
  course_title: string;
  created_at: string;
  event_type: string;
  reward_candidate_id: number;
  status: string;
  token_transaction: {
    amount: string;
    blockchain_address: string;
    chain_id: number | null;
    external_transaction_id: number;
    payout_transaction_id: number;
    recorded_at: string;
    transaction_hash: string | null;
  } | null;
  updated_at: string;
  wallet_credit: {
    amount: string;
    credited_at: string;
    internal_transaction_id: number;
    reward_wallet_credit_record_id: number;
    transaction_id: number;
    wallet_id: number;
  } | null;
};

export type WalletSummary = {
  id: number;
  organization_id: number | null;
  owner_type: "user" | "organization";
  user_id: number | null;
  value: string;
};

export type WalletLinkResult = {
  created: boolean;
  wallet: WalletSummary;
};

type LearnerErrorEnvelope = {
  error?: {
    code?: string;
    message?: string;
  };
};

export class LearnerRequestError extends Error {
  status: number;
  code: string;

  constructor(message: string, status: number, code: string) {
    super(message);
    this.name = "LearnerRequestError";
    this.status = status;
    this.code = code;
  }
}

export type LearnerRequestOptions = {
  apiRoot?: string;
  timeoutMs?: number;
  token: string;
};

export type RewardHistoryOptions = LearnerRequestOptions & {
  courseId?: number;
  limit?: number;
  offset?: number;
  status?: string;
};

const DEFAULT_TIMEOUT_MS = 10000;
const DEFAULT_REWARD_HISTORY_LIMIT = 25;

export async function fetchRewardHistory({
  apiRoot = "/api",
  courseId,
  limit = DEFAULT_REWARD_HISTORY_LIMIT,
  offset,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: RewardHistoryOptions): Promise<RewardHistoryEntry[]> {
  const query = new URLSearchParams();
  query.set("limit", String(limit));
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  if (typeof courseId === "number") {
    query.set("course_id", String(courseId));
  }
  if (status) {
    query.set("status", status);
  }

  return learnerJsonRequest<RewardHistoryEntry[]>({
    timeoutMs,
    token,
    url: `${apiRoot}/reward-candidates/me/history?${query.toString()}`,
  });
}

export async function fetchMyWallet({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: LearnerRequestOptions): Promise<WalletSummary | null> {
  const response = await learnerRawRequest({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/wallets/me`,
  });

  if (response.status === 404) {
    return null;
  }

  if (!response.ok) {
    throw await learnerErrorFromResponse(response, "Wallet could not be loaded.");
  }

  return (await response.json()) as WalletSummary;
}

export async function linkMyWallet({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: LearnerRequestOptions): Promise<WalletLinkResult> {
  return learnerJsonRequest<WalletLinkResult>({
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/wallets/me/link`,
  });
}

async function learnerJsonRequest<T>({
  method = "GET",
  timeoutMs,
  token,
  url,
}: {
  method?: string;
  timeoutMs: number;
  token: string;
  url: string;
}): Promise<T> {
  const response = await learnerRawRequest({ method, timeoutMs, token, url });
  if (!response.ok) {
    throw await learnerErrorFromResponse(response, "Learner request failed.");
  }

  return (await response.json()) as T;
}

async function learnerRawRequest({
  method,
  timeoutMs,
  token,
  url,
}: {
  method: string;
  timeoutMs: number;
  token: string;
  url: string;
}) {
  const trimmedToken = token.trim();
  if (!trimmedToken) {
    throw new LearnerRequestError("A sign-in token is required.", 401, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    return await fetch(url, {
      headers: {
        Authorization: `Bearer ${trimmedToken}`,
      },
      method,
      signal: controller.signal,
    });
  } catch (error) {
    if (error instanceof DOMException && error.name === "AbortError") {
      throw new LearnerRequestError("Learner request timed out.", 0, "timeout");
    }

    throw new LearnerRequestError("Learner request failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}

async function learnerErrorFromResponse(response: Response, fallbackMessage: string) {
  const fallbackCode =
    response.status === 401
      ? "unauthorized"
      : response.status === 403
        ? "permission_denied"
        : response.status === 404
          ? "not_found"
          : response.status === 409
            ? "conflict"
            : response.status >= 500
              ? "server_error"
              : "learner_error";
  const contentType = response.headers.get("content-type") || "";

  if (contentType.includes("application/json")) {
    const body = (await response.json()) as LearnerErrorEnvelope;
    return new LearnerRequestError(
      body.error?.message || fallbackMessage,
      response.status,
      body.error?.code || fallbackCode,
    );
  }

  const text = await response.text();
  return new LearnerRequestError(text || response.statusText || fallbackMessage, response.status, fallbackCode);
}
