import { DEFAULT_REWARD_HISTORY_LIMIT } from "./DEFAULT_REWARD_HISTORY_LIMIT";
import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type RewardHistoryEntry } from "./RewardHistoryEntry";
import { type RewardHistoryOptions } from "./RewardHistoryOptions";

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
