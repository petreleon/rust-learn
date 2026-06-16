import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { type WalletDepositIntentAudit } from "@/lib/learner";
import { WalletDepositIntentHistoryCard } from "../components/WalletDepositIntentHistoryCard";
import {
  walletDepositIntentNextStep,
  walletDepositIntentTone,
} from "../model/walletDepositIntentHistory";

describe("WalletDepositIntentHistoryCard", () => {
  it("shows pending and credited deposit intent states", () => {
    const { rerender } = render(<WalletDepositIntentHistoryCard intent={intent("pending_chain_confirmation")} />);

    expect(screen.getByText("pending chain confirmation")).toBeVisible();
    expect(screen.getByText("Waiting for the external wallet action and chain confirmation.")).toBeVisible();
    expect(screen.getByText("Token tx 0xhash123456")).toBeVisible();

    rerender(<WalletDepositIntentHistoryCard intent={intent("credited")} />);

    expect(screen.getByText("credited")).toBeVisible();
    expect(screen.getByText("Deposit was matched on-chain and credited to this wallet.")).toBeVisible();
  });

  it("shows retry and recreate guidance for ambiguous and failed states", () => {
    expect(walletDepositIntentTone("credited")).toBe("good");
    expect(walletDepositIntentTone("ambiguous")).toBe("warn");
    expect(walletDepositIntentTone("failed")).toBe("bad");

    expect(walletDepositIntentNextStep(intent("ambiguous"))).toContain("Recreate this deposit intent");
    expect(walletDepositIntentNextStep(intent("failed", { last_error: "indexer timeout" }))).toContain("Retry");
  });
});

function intent(status: string, overrides: Partial<WalletDepositIntentAudit> = {}): WalletDepositIntentAudit {
  return {
    amount: "20",
    chain_id: 31337,
    contract_address: "0xcontract",
    created_at: "2026-06-12T10:00:00Z",
    credited_at: status === "credited" ? "2026-06-12T10:05:00Z" : null,
    ethereum_address: "0xwallet",
    event_type: null,
    external_transaction_id: null,
    gas_payer: "platform",
    id: 8,
    last_error: null,
    log_index: 0,
    metamask_required: true,
    platform_address: "0xplatform",
    status,
    tax_amount: "2",
    transaction_hash: "0xhash123456789",
    transaction_id: null,
    updated_at: "2026-06-12T10:00:00Z",
    user_id: 7,
    wallet_action: "metamask_transfer",
    wallet_id: 3,
    wallet_provider: "metamask",
    ...overrides,
  };
}
