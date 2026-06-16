import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { type WalletSummary } from "@/lib/learner/WalletSummary";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type NotificationPreferences } from "@/lib/session/NotificationPreferences";
import { SessionRequestError } from "@/lib/session/SessionRequestError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  loadAccountKycStatus,
  loadAccountSettingsData,
  saveAccountNotificationPreferences,
} from "../api/accountSettingsApi";
import { AccountSettingsRoute } from "../route/AccountSettingsRoute";

vi.mock("next/navigation", () => ({
  usePathname: () => "/settings/account",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/accountSettingsApi", () => ({
  loadAccountKycStatus: vi.fn(),
  loadAccountSettingsData: vi.fn(),
  saveAccountNotificationPreferences: vi.fn(),
  submitAccountKyc: vi.fn(),
}));

describe("AccountSettingsRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("account-token");
    vi.mocked(loadAccountSettingsData).mockResolvedValue({
      prefs: preferences(),
      session: sessionFixture(),
      walletResult: { error: null, wallet: walletFixture() },
    });
    vi.mocked(loadAccountKycStatus).mockResolvedValue({
      next_action: "wait_for_review",
      submission: null,
      user_kyc_verified: false,
    });
    vi.mocked(saveAccountNotificationPreferences).mockResolvedValue(preferences({ push_enabled: true }));
  });

  it("shows the signed-out state without loading account APIs", async () => {
    vi.mocked(readBrowserSessionToken).mockReturnValue(null);

    render(<AccountSettingsRoute />);

    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
    expect(loadAccountSettingsData).not.toHaveBeenCalled();
  });

  it("loads account settings through the feature API boundary", async () => {
    render(<AccountSettingsRoute />);

    expect(await screen.findByRole("heading", { name: "Account" })).toBeVisible();
    expect((await screen.findAllByText("Ada Account"))[0]).toBeVisible();
    expect(await screen.findByText("42 LRN")).toBeVisible();
    await waitFor(() => expect(loadAccountSettingsData).toHaveBeenCalledWith({ token: "account-token" }));
    expect(loadAccountKycStatus).toHaveBeenCalledWith({ token: "account-token" });
  });

  it("saves notification preferences through the feature API boundary", async () => {
    const user = userEvent.setup();
    render(<AccountSettingsRoute />);

    expect(await screen.findByRole("heading", { name: "Notification preferences" })).toBeVisible();
    await user.click(screen.getByRole("checkbox", { name: /Reward status updates/ }));
    await user.click(screen.getByRole("button", { name: "Save preferences" }));

    await waitFor(() =>
      expect(saveAccountNotificationPreferences).toHaveBeenCalledWith({
        emailEnabled: true,
        pushEnabled: true,
        token: "account-token",
      }),
    );
    expect(await screen.findByText("Preferences saved.")).toBeVisible();
  });

  it("clears expired stored sessions", async () => {
    vi.mocked(loadAccountSettingsData).mockRejectedValue(new SessionRequestError("Expired session", 401, "unauthorized"));

    render(<AccountSettingsRoute />);

    expect(await screen.findByText("Session expired")).toBeVisible();
    expect(await screen.findByText("Expired session")).toBeVisible();
    expect(clearBrowserSession).toHaveBeenCalled();
  });

  it("clears stored session when signing out", async () => {
    const user = userEvent.setup();
    render(<AccountSettingsRoute />);

    expect((await screen.findAllByText("Ada Account"))[0]).toBeVisible();
    await user.click(screen.getAllByRole("button", { name: "Sign out" })[0]);

    expect(clearBrowserSession).toHaveBeenCalled();
    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
  });
});

function preferences(overrides: Partial<NotificationPreferences> = {}): NotificationPreferences {
  return {
    email_enabled: true,
    push_enabled: false,
    updated_at: "2026-06-12T10:00:00Z",
    user_id: 11,
    ...overrides,
  };
}

function walletFixture(): WalletSummary {
  return { id: 4, organization_id: null, owner_type: "user", user_id: 11, value: "42 LRN" };
}

function sessionFixture(): CurrentSession {
  return {
    access: { learner: true, organization: true, platform_admin: false, teacher: false, teacher_application: true },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      capabilities: [],
      delegated_permissions: [],
      direct_permissions: [],
      effective_permissions: [],
      roles: [],
    },
    user: {
      email: "ada@example.com",
      email_verified: true,
      id: 11,
      kyc_verified: false,
      name: "Ada Account",
    },
  };
}
