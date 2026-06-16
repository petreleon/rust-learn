import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { LearnerRequestError } from "@/lib/learner";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { createWalletTransfer } from "../api/walletTransferApi";
import { WalletTransferPanel } from "../route/WalletTransferPanel";

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/walletTransferApi", () => ({
  createWalletTransfer: vi.fn(),
}));

describe("WalletTransferPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("wallet-token");
  });

  it("keeps wallet transfers locked until wallet and KYC gates are ready", async () => {
    render(<WalletTransferPanel enabled={false} walletLinked={false} onRefresh={vi.fn()} />);

    expect(screen.getByText("Locked")).toBeVisible();
    expect(screen.getByRole("button", { name: "Create deposit" })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Create retirement" })).toBeDisabled();
  });

  it("creates a deposit intent and shows tax, delta, receiver, and wallet action", async () => {
    const user = userEvent.setup();
    const onRefresh = vi.fn();
    vi.mocked(createWalletTransfer).mockResolvedValue({
      amount: "20",
      chain_id: 31337,
      contract_address: "0xcontract",
      ethereum_address: "0xwallet",
      gas_payer: "platform",
      id: 8,
      log_index: 0,
      metamask_required: true,
      operation: "deposit",
      platform_address: "0xplatform",
      status: "pending_chain_confirmation",
      tax_amount: "2",
      transaction_hash: "0xhash",
      wallet_action: "metamask_permit_signature",
      wallet_delta_on_confirmation: "18",
      wallet_id: 3,
      wallet_provider: "metamask",
    });

    render(<WalletTransferPanel enabled walletLinked onRefresh={onRefresh} />);
    const depositForm = screen.getByRole("heading", { name: "Deposit intent" }).closest("form");
    expect(depositForm).not.toBeNull();

    await user.type(within(depositForm!).getByLabelText("Amount"), "20");
    await user.type(within(depositForm!).getByLabelText("Ethereum address"), "0xwallet");
    await user.click(within(depositForm!).getByRole("button", { name: "Create deposit" }));

    await waitFor(() =>
      expect(createWalletTransfer).toHaveBeenCalledWith({
        draft: expect.objectContaining({ amount: "20", ethereumAddress: "0xwallet" }),
        operation: "deposit",
        token: "wallet-token",
      }),
    );
    expect(onRefresh).toHaveBeenCalled();
    expect(await screen.findByText("Deposit intent created")).toBeVisible();
    expect(screen.getByText("2 tax")).toBeVisible();
    expect(screen.getByText("18 wallet delta")).toBeVisible();
    expect(screen.getByText("Receiver 0xplatform")).toBeVisible();
    expect(screen.getByText("metamask permit signature")).toBeVisible();
  });

  it("shows retirement backend errors without clearing local form state", async () => {
    const user = userEvent.setup();
    vi.mocked(createWalletTransfer).mockRejectedValue(
      new LearnerRequestError("Insufficient wallet balance", 409, "insufficient_funds"),
    );

    render(<WalletTransferPanel enabled walletLinked onRefresh={vi.fn()} />);
    const retireForm = screen.getByRole("heading", { name: "Retire tokens" }).closest("form");
    expect(retireForm).not.toBeNull();

    await user.type(within(retireForm!).getByLabelText("Amount"), "999");
    await user.type(within(retireForm!).getByLabelText("Ethereum address"), "0xwallet");
    await user.click(within(retireForm!).getByRole("button", { name: "Create retirement" }));

    expect(await screen.findByText("Insufficient wallet balance")).toBeVisible();
    expect(within(retireForm!).getByLabelText("Amount")).toHaveValue(999);
  });
});
