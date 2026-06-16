import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { AdminDashboardRoute } from "../route/AdminDashboardRoute";
import {
  downloadAdminDashboardCsv,
  loadAdminDashboardSession,
  loadPlatformFraudControls,
  loadPlatformRewardOperations,
  loadPlatformSummary,
  loadPlatformSystemStatus,
} from "../api/dashboardApi";
import { startDashboardCsvDownload } from "../route/startDashboardCsvDownload";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { adminSession, fraudDashboard, rewardDashboard, systemStatus } from "./adminDashboardTestFixtures";

vi.mock("next/navigation", () => ({
  usePathname: () => "/admin",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/dashboardApi", () => ({
  downloadAdminDashboardCsv: vi.fn(),
  loadAdminDashboardSession: vi.fn(),
  loadPlatformFraudControls: vi.fn(),
  loadPlatformRewardOperations: vi.fn(),
  loadPlatformSummary: vi.fn(),
  loadPlatformSystemStatus: vi.fn(),
}));

vi.mock("../route/startDashboardCsvDownload", () => ({
  startDashboardCsvDownload: vi.fn(),
}));

function mockToken(token: string | null) {
  vi.mocked(readBrowserSessionToken).mockReturnValue(token);
}

describe("AdminDashboardRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockToken("admin-token");
    vi.mocked(loadAdminDashboardSession).mockResolvedValue(
      adminSession(["VIEW_REPORT", "VIEW_REWARD_AUDIT", "MANAGE_REWARD_FRAUD_BLOCKS", "EXPORT_DATA"]),
    );
    vi.mocked(loadPlatformSummary).mockResolvedValue({
      total_courses: 7,
      total_notifications: 11,
      total_organizations: 3,
      total_users: 42,
      total_wallets: 5,
    });
    vi.mocked(loadPlatformRewardOperations).mockResolvedValue(rewardDashboard());
    vi.mocked(loadPlatformFraudControls).mockResolvedValue(fraudDashboard());
    vi.mocked(loadPlatformSystemStatus).mockResolvedValue(systemStatus());
    vi.mocked(downloadAdminDashboardCsv).mockResolvedValue({
      body: "id,total\n1,42\n",
      filename: "summary.csv",
    });
  });

  it("shows the signed-out state without loading dashboard APIs", async () => {
    mockToken(null);

    render(<AdminDashboardRoute />);

    expect(await screen.findByText("Sign in required")).toBeVisible();
    expect(loadAdminDashboardSession).not.toHaveBeenCalled();
    expect(loadPlatformSummary).not.toHaveBeenCalled();
  });

  it("loads dashboard sections through the feature API boundary", async () => {
    render(<AdminDashboardRoute />);

    expect(await screen.findByText("Platform admin dashboard")).toBeVisible();
    expect(await screen.findByText("Users")).toBeVisible();
    expect(await screen.findByText("42")).toBeVisible();
    expect(await screen.findByText("Reward operations")).toBeVisible();
    expect((await screen.findAllByText("Fraud controls")).length).toBeGreaterThan(0);
    expect(await screen.findByText("System status")).toBeVisible();
    expect(loadAdminDashboardSession).toHaveBeenCalledWith({ token: "admin-token" });
    expect(loadPlatformSummary).toHaveBeenCalledWith({ token: "admin-token" });
    expect(loadPlatformRewardOperations).toHaveBeenCalledWith({ token: "admin-token" });
    expect(loadPlatformFraudControls).toHaveBeenCalledWith({ token: "admin-token" });
    expect(loadPlatformSystemStatus).toHaveBeenCalled();
  });

  it("downloads CSV exports through the dashboard action controller", async () => {
    const user = userEvent.setup();

    render(<AdminDashboardRoute />);

    await user.click((await screen.findAllByRole("button", { name: /^CSV$/ }))[0]);

    await waitFor(() =>
      expect(downloadAdminDashboardCsv).toHaveBeenCalledWith({
        report: "summary",
        token: "admin-token",
      }),
    );
    expect(startDashboardCsvDownload).toHaveBeenCalledWith({
      body: "id,total\n1,42\n",
      filename: "summary.csv",
    });
    expect(await screen.findByText("CSV ready")).toBeVisible();
  });

  it("shows the admin access gate before loading dashboard sections", async () => {
    vi.mocked(loadAdminDashboardSession).mockResolvedValue(adminSession([]));

    render(<AdminDashboardRoute />);

    expect(await screen.findByText("Platform admin access is not available")).toBeVisible();
    expect(loadPlatformSummary).not.toHaveBeenCalled();
    expect(loadPlatformRewardOperations).not.toHaveBeenCalled();
    expect(loadPlatformFraudControls).not.toHaveBeenCalled();
  });
});
