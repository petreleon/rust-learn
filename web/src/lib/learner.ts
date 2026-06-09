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

export type CourseCatalogResponse = {
  courses: CourseCatalogItem[];
  enrollment_status: string | null;
  lifecycle_status: string | null;
  limit: number;
  offset: number;
  organization_id: number | null;
  reward_available: boolean | null;
  search: string | null;
  total: number;
};

export type CourseCatalogItem = {
  access: CourseAccessSummary;
  content: CourseContentSummary;
  description: string | null;
  enrollment: CourseEnrollmentSummary;
  id: number;
  lifecycle_status: string;
  organizations: CourseCatalogOrganization[];
  rewards: CourseRewardSummary;
  teachers: CourseCatalogTeacher[];
  title: string;
  topics: string[];
};

export type CourseCatalogDetail = {
  chapters: CourseCatalogChapter[];
  course: CourseCatalogItem;
  prerequisites: string[];
};

export type LearnerDashboardSnapshot = {
  enrolled_catalog: CourseCatalogResponse;
  recommended_catalog: CourseCatalogResponse;
  reward_history: RewardHistoryEntry[];
  wallet: WalletSummary | null;
};

export type CourseLearningResponse = {
  active_content_id: number | null;
  chapters: CourseLearningChapter[];
  course: CourseCatalogItem;
  progress_supported: boolean;
};

export type CourseCatalogOrganization = {
  id: number;
  name: string;
};

export type CourseCatalogTeacher = {
  id: number;
  name: string;
};

export type CourseContentSummary = {
  chapter_count: number;
  content_count: number;
  content_types: string[];
  has_content: boolean;
};

export type CourseRewardSummary = {
  active_policy_count: number;
  available: boolean;
  event_types: string[];
  payment_strategies: string[];
  token_amounts: string[];
};

export type CourseEnrollmentSummary = {
  can_request_join: boolean;
  reason: string | null;
  request_id: number | null;
  roles: string[];
  state: string;
};

export type CourseAccessSummary = {
  can_request_join: boolean;
  can_view_content: boolean;
  can_view_course: boolean;
  can_view_rewards: boolean;
};

export type CourseCatalogChapter = {
  contents: CourseCatalogContent[];
  id: number;
  order: number;
  title: string;
};

export type CourseCatalogContent = {
  content_type: string;
  id: number;
  order: number;
};

export type CourseLearningChapter = {
  contents: CourseLearningContent[];
  id: number;
  order: number;
  title: string;
};

export type CourseLearningContent = {
  chapter_id: number;
  content_type: string;
  data: string | null;
  display_state: string;
  id: number;
  order: number;
  processing_error: string | null;
  processing_status: string | null;
};

export type CourseJoinRequest = {
  course_id: number;
  created_at: string;
  decided_at: string | null;
  decision_reason: string | null;
  id: number;
  requester_user_id: number;
  reviewer_user_id: number | null;
  status: string;
  updated_at: string;
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

export type CourseCatalogOptions = LearnerRequestOptions & {
  enrollmentStatus?: string;
  limit?: number;
  offset?: number;
  organizationId?: number;
  rewardAvailable?: boolean;
  search?: string;
};

export type CourseDetailOptions = LearnerRequestOptions & {
  courseId: number;
};

const DEFAULT_TIMEOUT_MS = 10000;
const DEFAULT_COURSE_CATALOG_LIMIT = 30;
const DEFAULT_REWARD_HISTORY_LIMIT = 25;
const DEFAULT_DASHBOARD_COURSE_LIMIT = 6;
const DEFAULT_DASHBOARD_RECOMMENDED_LIMIT = 3;
const DEFAULT_DASHBOARD_REWARD_LIMIT = 6;

export async function fetchLearnerDashboard({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: LearnerRequestOptions): Promise<LearnerDashboardSnapshot> {
  const [enrolledCatalog, recommendedCatalog, rewardHistory, wallet] = await Promise.all([
    fetchCourseCatalog({
      apiRoot,
      enrollmentStatus: "enrolled",
      limit: DEFAULT_DASHBOARD_COURSE_LIMIT,
      timeoutMs,
      token,
    }),
    fetchCourseCatalog({
      apiRoot,
      enrollmentStatus: "available",
      limit: DEFAULT_DASHBOARD_RECOMMENDED_LIMIT,
      timeoutMs,
      token,
    }),
    fetchRewardHistory({
      apiRoot,
      limit: DEFAULT_DASHBOARD_REWARD_LIMIT,
      timeoutMs,
      token,
    }),
    fetchMyWallet({
      apiRoot,
      timeoutMs,
      token,
    }),
  ]);

  return {
    enrolled_catalog: enrolledCatalog,
    recommended_catalog: recommendedCatalog,
    reward_history: rewardHistory,
    wallet,
  };
}

export async function fetchCourseCatalog({
  apiRoot = "/api",
  enrollmentStatus,
  limit = DEFAULT_COURSE_CATALOG_LIMIT,
  offset,
  organizationId,
  rewardAvailable,
  search,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseCatalogOptions): Promise<CourseCatalogResponse> {
  const query = new URLSearchParams();
  query.set("limit", String(limit));
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  if (typeof organizationId === "number") {
    query.set("organization_id", String(organizationId));
  }
  if (typeof rewardAvailable === "boolean") {
    query.set("reward_available", String(rewardAvailable));
  }
  const trimmedSearch = search?.trim();
  if (trimmedSearch) {
    query.set("search", trimmedSearch);
  }
  if (enrollmentStatus && enrollmentStatus !== "all") {
    query.set("enrollment_status", enrollmentStatus);
  }

  const suffix = query.toString();
  return learnerJsonRequest<CourseCatalogResponse>({
    timeoutMs,
    token,
    url: `${apiRoot}/courses/catalog${suffix ? `?${suffix}` : ""}`,
  });
}

export async function fetchCourseDetail({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions): Promise<CourseCatalogDetail> {
  return learnerJsonRequest<CourseCatalogDetail>({
    timeoutMs,
    token,
    url: `${apiRoot}/courses/catalog/${courseId}`,
  });
}

export async function fetchCourseLearning({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions): Promise<CourseLearningResponse> {
  return learnerJsonRequest<CourseLearningResponse>({
    timeoutMs,
    token,
    url: `${apiRoot}/courses/catalog/${courseId}/learn`,
  });
}

export async function requestCourseJoin({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions): Promise<CourseJoinRequest> {
  return learnerJsonRequest<CourseJoinRequest>({
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/join-requests`,
  });
}

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
