import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  createOrganizationTokenBurn,
  fetchOrganizationBurnPermissions,
  fetchOrganizationTokenBurns,
} from "../api/organizationBurnApi";
import { OrganizationBurnPanel } from "../route/OrganizationBurnPanel";

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/organizationBurnApi", () => ({
  createOrganizationTokenBurn: vi.fn(),
  fetchOrganizationBurnPermissions: vi.fn(),
  fetchOrganizationTokenBurns: vi.fn(),
}));

describe("OrganizationBurnPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("org-token");
  });

  it("keeps organization burns locked until the wallet exists", () => {
    render(<OrganizationBurnPanel organizationId={3} walletLinked={false} onRefresh={vi.fn()} />);

    expect(screen.getByText("Wallet required")).toBeVisible();
    expect(screen.getByRole("button", { name: "Request burn" })).toBeDisabled();
    expect(fetchOrganizationBurnPermissions).not.toHaveBeenCalled();
  });

  it("loads permissions and creates a centralized organization burn", async () => {
    const user = userEvent.setup();
    const onRefresh = vi.fn();
    vi.mocked(fetchOrganizationBurnPermissions).mockResolvedValue({
      can_burn: true,
      can_request_burn: true,
      kyc_verified: true,
      organization_id: 3,
      required_permission: "BURN_ORGANIZATION_TOKENS",
    });
    vi.mocked(fetchOrganizationTokenBurns).mockResolvedValue([]);
    vi.mocked(createOrganizationTokenBurn).mockResolvedValue(sampleBurn({ amount: "12" }));

    render(<OrganizationBurnPanel organizationId={3} walletLinked onRefresh={onRefresh} />);

    expect(await screen.findByText("Available")).toBeVisible();
    const form = screen.getByRole("button", { name: "Request burn" }).closest("form");
    expect(form).not.toBeNull();
    await user.type(within(form!).getByLabelText("Amount"), "12");
    await user.click(within(form!).getByRole("button", { name: "Request burn" }));

    await waitFor(() =>
      expect(createOrganizationTokenBurn).toHaveBeenCalledWith({
        draft: expect.objectContaining({ amount: "12", source: "centralized_wallet" }),
        idempotencyKey: expect.stringContaining("org-3-burn-"),
        organizationId: 3,
        token: "org-token",
      }),
    );
    expect(onRefresh).toHaveBeenCalled();
    expect(await screen.findByText("12 LearnToken")).toBeVisible();
  });

  it("shows permission and KYC gates before enabling submission", async () => {
    vi.mocked(fetchOrganizationBurnPermissions).mockResolvedValue({
      can_burn: false,
      can_request_burn: false,
      kyc_verified: true,
      organization_id: 3,
      required_permission: "BURN_ORGANIZATION_TOKENS",
    });
    vi.mocked(fetchOrganizationTokenBurns).mockResolvedValue([sampleBurn({ amount: "5" })]);

    render(<OrganizationBurnPanel organizationId={3} walletLinked onRefresh={vi.fn()} />);

    expect(await screen.findByText("Permission required")).toBeVisible();
    expect(screen.getByText("5 LearnToken")).toBeVisible();
    expect(screen.getByRole("button", { name: "Request burn" })).toBeDisabled();
  });
});

function sampleBurn({ amount }: { amount: string }) {
  return {
    actor_user_id: 7,
    amount,
    created_at: "2026-06-17T00:00:00Z",
    fee_amount: "0",
    fee_path: "none",
    id: Number(amount),
    internal_transaction_id: 9,
    leaderboard_visible: true,
    organization_id: 3,
    source: "centralized_wallet",
    status: "leaderboard_indexed",
    transaction_id: 8,
    wallet_action: "platform_burn_from_allowance",
  };
}
