import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type FraudBlockItem } from "./FraudBlockItem";
import { type FraudBlockRevokeOptions } from "./FraudBlockRevokeOptions";

export async function revokeFraudBlock({
  apiRoot = "/api",
  blockId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: FraudBlockRevokeOptions): Promise<FraudBlockItem> {
  return adminJsonRequest({
    apiRoot,
    method: "PUT",
    path: `/reward-fraud-blocks/${blockId}/revoke`,
    timeoutMs,
    token,
  });
}
