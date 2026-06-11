import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type FraudBlockItem } from "./FraudBlockItem";
import { type FraudBlockListOptions } from "./FraudBlockListOptions";

export async function fetchFraudBlocks({
  apiRoot = "/api",
  scope_type,
  active,
  limit,
  offset,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: FraudBlockListOptions): Promise<{ blocks: FraudBlockItem[]; limit: number; offset: number; total: number }> {
  const query = new URLSearchParams();
  if (scope_type) {
    query.set("scope_type", scope_type);
  }
  if (typeof active === "boolean") {
    query.set("active", String(active));
  }
  if (typeof limit === "number") {
    query.set("limit", String(limit));
  }
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  const suffix = query.toString();
  return adminJsonRequest({
    apiRoot,
    path: `/reward-fraud-blocks${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}
