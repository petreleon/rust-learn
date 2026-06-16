import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { createMyWalletBurn, fetchMyWalletBurns } from "../api/walletBurnApi";
import { WalletBurnPanel } from "../route/WalletBurnPanel";

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/walletBurnApi", () => ({
  createMyWalletBurn: vi.fn(),
  fetchMyWalletBurns: vi.fn(),
}));

describe("WalletBurnPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("learner-token");
  });

  it("keeps burns locked until wallet and KYC are ready", () => {
    render(<WalletBurnPanel enabled={false} walletLinked={false} onRefresh={vi.fn()} />);

    expect(screen.getByText("Wallet required")).toBeVisible();
    expect(screen.getByRole("button", { name: "Request burn" })).toBeDisabled();
    expect(fetchMyWalletBurns).not.toHaveBeenCalled();
  });

  it("creates a centralized wallet burn and refreshes wallet data", async () => {
    const user = userEvent.setup();
    const onRefresh = vi.fn();
    vi.mocked(fetchMyWalletBurns).mockResolvedValue([]);
    vi.mocked(createMyWalletBurn).mockResolvedValue(sampleBurn({ amount: "11" }));

    render(<WalletBurnPanel enabled walletLinked onRefresh={onRefresh} />);

    expect(await screen.findByText("Available")).toBeVisible();
    const form = screen.getByRole("button", { name: "Request burn" }).closest("form");
    expect(form).not.toBeNull();
    await user.type(within(form!).getByLabelText("Amount"), "11");
    await user.click(within(form!).getByRole("button", { name: "Request burn" }));

    await waitFor(() =>
      expect(createMyWalletBurn).toHaveBeenCalledWith({
        draft: expect.objectContaining({ amount: "11", source: "centralized_wallet" }),
        idempotencyKey: expect.stringContaining("user-burn-"),
        token: "learner-token",
      }),
    );
    expect(onRefresh).toHaveBeenCalled();
    expect(await screen.findByText("11 LearnToken burned")).toBeVisible();
  });

  it("shows MetaMask direct fee path and requires transaction evidence", async () => {
    const user = userEvent.setup();
    vi.mocked(fetchMyWalletBurns).mockResolvedValue([sampleBurn({ amount: "3" })]);

    render(<WalletBurnPanel enabled walletLinked onRefresh={vi.fn()} />);

    const form = screen.getByRole("button", { name: "Request burn" }).closest("form");
    expect(form).not.toBeNull();
    await user.type(within(form!).getByLabelText("Amount"), "4");
    await user.selectOptions(within(form!).getByLabelText("Burn source"), "decentralized_direct");
    await user.type(within(form!).getByLabelText("Ethereum address"), "0xwallet");

    expect(within(form!).getByDisplayValue("network fee paid by user")).toBeVisible();
    expect(within(form!).getByRole("button", { name: "Request burn" })).toBeDisabled();

    await user.click(within(form!).getByText("Chain evidence"));
    await user.type(within(form!).getByLabelText("Transaction hash"), "0xtx");
    expect(within(form!).getByRole("button", { name: "Request burn" })).toBeEnabled();
    expect(await screen.findByText("3 LearnToken burned")).toBeVisible();
  });
});

function sampleBurn({ amount }: { amount: string }) {
  return {
    amount,
    created_at: "2026-06-17T00:00:00Z",
    deposit_intent_id: null,
    external_transaction_id: null,
    fee_amount: "0",
    fee_path: "none",
    id: Number(amount),
    internal_transaction_id: 9,
    last_error: null,
    leaderboard_visible: true,
    metamask_required: false,
    source: "centralized_wallet",
    status: "leaderboard_indexed",
    transaction_id: 8,
    wallet_action: "platform_burn_from_allowance",
  };
}
