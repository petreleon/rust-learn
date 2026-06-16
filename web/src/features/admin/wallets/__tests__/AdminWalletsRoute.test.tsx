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
} from "../api/walletsApi";
import { startWalletCreditsCsvDownload } from "../route/startWalletCreditsCsvDownload";
import { adminWalletSession, walletReconciliation, walletSummary } from "./adminWalletsTestFixtures";

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
}));

vi.mock("../route/startWalletCreditsCsvDownload", () => ({
  startWalletCreditsCsvDownload: vi.fn(),
}));

function mockToken(token: string | null) {
  vi.mocked(readBrowserSessionToken).mockReturnValue(token);
}

describe("AdminWalletsRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockToken("admin-token");
    vi.mocked(loadAdminWalletSession).mockResolvedValue(
      adminWalletSession(["VIEW_TRANSACTIONS", "EXPORT_DATA"]),
    );
    vi.mocked(loadAdminWalletSummary).mockResolvedValue(walletSummary());
    vi.mocked(loadAdminWalletReconciliation).mockResolvedValue(walletReconciliation());
    vi.mocked(downloadWalletCreditsCsv).mockResolvedValue({
      body: "wallet_id,balance\n10,150.00\n",
      filename: "wallet_credits.csv",
    });
  });

  it("shows the signed-out state without loading wallet APIs", async () => {
    mockToken(null);

    render(<AdminWalletsRoute />);

    expect(await screen.findByText("Sign in required")).toBeVisible();
    expect(loadAdminWalletSession).not.toHaveBeenCalled();
    expect(loadAdminWalletSummary).not.toHaveBeenCalled();
    expect(loadAdminWalletReconciliation).not.toHaveBeenCalled();
  });

  it("loads wallet summary and reconciliation through the feature API", async () => {
    render(<AdminWalletsRoute />);

    expect(await screen.findByText("Wallet audit")).toBeVisible();
    expect(await screen.findByText("Total wallets")).toBeVisible();
    expect((await screen.findAllByText("12")).length).toBeGreaterThan(0);
    expect(await screen.findByText("Wallet reconciliation")).toBeVisible();
    expect(await screen.findByText("Wallet 10")).toBeVisible();
    expect(loadAdminWalletSession).toHaveBeenCalledWith({ token: "admin-token" });
    expect(loadAdminWalletSummary).toHaveBeenCalledWith({ token: "admin-token" });
    expect(loadAdminWalletReconciliation).toHaveBeenCalledWith({ token: "admin-token" });
  });

  it("downloads wallet credits through the wallet action controller", async () => {
    const user = userEvent.setup();

    render(<AdminWalletsRoute />);

    await user.click(await screen.findByRole("button", { name: "Download wallet credits CSV" }));

    await waitFor(() => expect(downloadWalletCreditsCsv).toHaveBeenCalledWith({ token: "admin-token" }));
    expect(startWalletCreditsCsvDownload).toHaveBeenCalledWith({
      body: "wallet_id,balance\n10,150.00\n",
      filename: "wallet_credits.csv",
    });
  });

  it("shows the wallet permission gate before loading wallet data", async () => {
    vi.mocked(loadAdminWalletSession).mockResolvedValue(adminWalletSession(["EXPORT_DATA"]));

    render(<AdminWalletsRoute />);

    expect(await screen.findByText("Wallet audit unavailable")).toBeVisible();
    expect(loadAdminWalletSummary).not.toHaveBeenCalled();
    expect(loadAdminWalletReconciliation).not.toHaveBeenCalled();
  });
});
