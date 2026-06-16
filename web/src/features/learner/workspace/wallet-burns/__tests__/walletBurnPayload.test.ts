import { describe, expect, it } from "vitest";
import { walletBurnDraftIsReady, walletBurnPayload } from "../model/walletBurnPayload";
import { defaultWalletBurnDraft } from "../model/walletBurnTypes";

describe("walletBurnPayload", () => {
  it("maps centralized burns to fee-free internal wallet requests", () => {
    const draft = { ...defaultWalletBurnDraft(), amount: "12" };

    expect(walletBurnDraftIsReady(draft)).toBe(true);
    expect(walletBurnPayload(draft, "key")).toMatchObject({
      amount: "12",
      fee_path: "none",
      idempotency_key: "key",
      source: "centralized_wallet",
    });
  });

  it("requires direct MetaMask burns to include confirmed transaction evidence", () => {
    const draft = {
      ...defaultWalletBurnDraft(),
      amount: "5",
      ethereumAddress: "0xwallet",
      source: "decentralized_direct" as const,
    };

    expect(walletBurnDraftIsReady(draft)).toBe(false);
    expect(walletBurnDraftIsReady({ ...draft, transactionHash: "0xtx" })).toBe(true);
    expect(walletBurnPayload({ ...draft, transactionHash: "0xtx" }, "key")).toMatchObject({
      ethereum_address: "0xwallet",
      fee_path: "network_fee_paid_by_user",
      transaction_hash: "0xtx",
    });
  });
});
