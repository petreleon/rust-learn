import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { AdminWalletsRoute } from "../route/AdminWalletsRoute";
import {
  downloadWalletCreditsCsv,
  loadAdminWalletReconciliation,
  loadAdminWalletSession,
  loadAdminWalletSummary,
  loadBurnLeaderboard,
  loadWalletTokenTaxAudit,
  loadWalletTokenTaxes,
  saveWalletTokenTax,
} from "../api/walletsApi";
import { startWalletCreditsCsvDownload } from "../route/startWalletCreditsCsvDownload";
import { adminWalletSession, burnLeaderboard } from "./adminWalletsTestFixtures";

vi.mock("next/navigation", () => ({
  usePathname: () => "/admin/wallets",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/walletsApi", () => ({
  downloadWalletCreditsCsv: vi.fn(),
  loadAdminWalletReconciliation: vi.fn(),
  loadAdminWalletSession: vi.fn(),
  loadAdminWalletSummary: vi.fn(),
  loadBurnLeaderboard: vi.fn(),
  loadWalletTokenTaxAudit: vi.fn(),
  loadWalletTokenTaxes: vi.fn(),
  saveWalletTokenTax: vi.fn(),
}));

vi.mock("../route/startWalletCreditsCsvDownload", () => ({
  startWalletCreditsCsvDownload: vi.fn(),
}));

describe("AdminWalletBurnLeaderboardRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("admin-token");
    vi.mocked(loadAdminWalletSession).mockResolvedValue(
      adminWalletSession(["VIEW_BURN_LEADERBOARD"]),
    );
    vi.mocked(loadBurnLeaderboard).mockResolvedValue(burnLeaderboard());
    vi.mocked(saveWalletTokenTax).mockResolvedValue({
      operation: "deposit",
      tax_amount: "3",
    });
    vi.mocked(downloadWalletCreditsCsv).mockResolvedValue({
      body: "wallet_id,balance\n10,150.00\n",
      filename: "wallet_credits.csv",
    });
  });

  it("loads burn leaderboard for leaderboard administrators", async () => {
    render(<AdminWalletsRoute />);

    expect(await screen.findByText("Burn leaderboard")).toBeVisible();
    expect(await screen.findByText("#1 User 44")).toBeVisible();
    expect(loadBurnLeaderboard).toHaveBeenCalledWith({
      scope: "all",
      token: "admin-token",
      window: "7d",
    });
    expect(loadAdminWalletSummary).not.toHaveBeenCalled();
    expect(loadAdminWalletReconciliation).not.toHaveBeenCalled();
    expect(loadWalletTokenTaxAudit).not.toHaveBeenCalled();
    expect(loadWalletTokenTaxes).not.toHaveBeenCalled();
    expect(startWalletCreditsCsvDownload).not.toHaveBeenCalled();
  });

  it("reloads burn leaderboard when filters change", async () => {
    const user = userEvent.setup();

    render(<AdminWalletsRoute />);

    await screen.findByText("Burn leaderboard");
    await user.selectOptions(screen.getByLabelText("Burn leaderboard window"), "30d");
    await waitFor(() =>
      expect(loadBurnLeaderboard).toHaveBeenLastCalledWith({
        scope: "all",
        token: "admin-token",
        window: "30d",
      }),
    );

    await user.selectOptions(screen.getByLabelText("Burn leaderboard scope"), "organizations");
    await waitFor(() =>
      expect(loadBurnLeaderboard).toHaveBeenLastCalledWith({
        scope: "organizations",
        token: "admin-token",
        window: "30d",
      }),
    );
  });
});
