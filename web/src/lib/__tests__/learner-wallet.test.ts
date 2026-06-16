import { afterEach, describe, expect, it, vi } from "vitest";
import { fetchLearnerWallet } from "@/lib/learner";

describe("fetchLearnerWallet", () => {
  afterEach(() => vi.unstubAllGlobals());

  it("loads persisted wallet history from the wallet audit endpoint", async () => {
    const fetchMock = vi.fn(async (url: string) => {
      if (url.endsWith("/wallets/me")) {
        return Response.json({ id: 3, organization_id: null, owner_type: "user", user_id: 7, value: "20" });
      }
      if (url.includes("/reward-candidates/me/history")) {
        return Response.json([]);
      }
      if (url.endsWith("/wallets/me/audit")) {
        return Response.json({
          compensation_records: [],
          deposit_intents: [depositIntent("pending_chain_confirmation")],
          external_transactions: [],
          internal_transactions: [],
          reward_records: [],
          wallet: { id: 3, organization_id: null, owner_type: "user", user_id: 7, value: "20" },
        });
      }
      throw new Error(`Unexpected URL ${url}`);
    });
    vi.stubGlobal("fetch", fetchMock);

    const snapshot = await fetchLearnerWallet({ apiRoot: "http://api.test", token: "token" });

    expect(snapshot.wallet_history).toHaveLength(1);
    expect(snapshot.wallet_history[0]?.status).toBe("pending_chain_confirmation");
    expect(fetchMock).toHaveBeenCalledWith(
      "http://api.test/wallets/me/audit",
      expect.objectContaining({ headers: { Authorization: "Bearer token" }, method: "GET" }),
    );
  });

  it("does not fetch audit history when the learner wallet is missing", async () => {
    const fetchMock = vi.fn(async (url: string) => {
      if (url.endsWith("/wallets/me")) return new Response("Not found", { status: 404 });
      if (url.includes("/reward-candidates/me/history")) return Response.json([]);
      throw new Error(`Unexpected URL ${url}`);
    });
    vi.stubGlobal("fetch", fetchMock);

    const snapshot = await fetchLearnerWallet({ apiRoot: "http://api.test", token: "token" });

    expect(snapshot.wallet).toBeNull();
    expect(snapshot.wallet_history).toEqual([]);
    expect(fetchMock).not.toHaveBeenCalledWith(expect.stringContaining("/wallets/me/audit"), expect.anything());
  });
});

function depositIntent(status: string) {
  return {
    amount: "20",
    chain_id: 31337,
    contract_address: "0xcontract",
    created_at: "2026-06-12T10:00:00Z",
    credited_at: null,
    ethereum_address: "0xwallet",
    event_type: null,
    external_transaction_id: null,
    gas_payer: "platform",
    id: 8,
    last_error: null,
    log_index: null,
    metamask_required: true,
    platform_address: "0xplatform",
    status,
    tax_amount: "2",
    transaction_hash: "0xhash",
    transaction_id: null,
    updated_at: "2026-06-12T10:00:00Z",
    user_id: 7,
    wallet_action: "metamask_transfer",
    wallet_id: 3,
    wallet_provider: "metamask",
  };
}
