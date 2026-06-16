import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type OrganizationDetail } from "@/lib/organization/OrganizationDetail";
import { OrganizationSettingsRoute } from "../route/OrganizationSettingsRoute";
import {
  loadCurrentOrganizationSession,
  loadOrganizationSettings,
  saveOrganizationSettings,
} from "../api/settingsApi";

vi.mock("next/navigation", () => ({
  usePathname: () => "/organizations/4/settings",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));
vi.mock("../api/settingsApi", () => ({
  deleteOrganizationSettings: vi.fn(),
  loadCurrentOrganizationSession: vi.fn(),
  loadOrganizationSettings: vi.fn(),
  saveOrganizationSettings: vi.fn(),
}));

const emptyScope = { capabilities: [], delegated_permissions: [], direct_permissions: [], effective_permissions: [], roles: [] };

describe("OrganizationSettingsRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("org-token");
    vi.mocked(loadCurrentOrganizationSession).mockResolvedValue(sessionFixture());
    vi.mocked(loadOrganizationSettings).mockResolvedValue(detailFixture());
    vi.mocked(saveOrganizationSettings).mockResolvedValue({ ...detailFixture(), name: "Ferris Labs" });
  });

  it("shows signed-out state without loading settings", async () => {
    vi.mocked(readBrowserSessionToken).mockReturnValue(null);

    render(<OrganizationSettingsRoute organizationId="4" />);

    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
    expect(loadCurrentOrganizationSession).not.toHaveBeenCalled();
    expect(loadOrganizationSettings).not.toHaveBeenCalled();
  });

  it("loads settings through the feature API", async () => {
    render(<OrganizationSettingsRoute organizationId="4" />);

    expect(await screen.findByRole("heading", { name: "Organization identity" })).toBeVisible();
    expect(screen.getByPlaceholderText("Organization name")).toHaveValue("Ferris Org");
    expect(loadOrganizationSettings).toHaveBeenCalledWith({
      organizationId: 4,
      token: "org-token",
    });
  });

  it("saves settings through the controller", async () => {
    const user = userEvent.setup();
    render(<OrganizationSettingsRoute organizationId="4" />);

    await screen.findByRole("heading", { name: "Organization identity" });
    const name = screen.getByPlaceholderText("Organization name");
    await user.clear(name);
    await user.type(name, "Ferris Labs");
    await user.click(screen.getByRole("button", { name: "Save settings" }));

    await waitFor(() => expect(saveOrganizationSettings).toHaveBeenCalled());
    expect(saveOrganizationSettings).toHaveBeenCalledWith({
      organizationId: 4,
      payload: {
        name: "Ferris Labs",
        profile_url: null,
        website_link: "https://ferris.example",
      },
      token: "org-token",
    });
    expect(await screen.findByText("Settings saved.")).toBeVisible();
  });
});

function sessionFixture(): CurrentSession {
  return {
    access: { learner: true, organization: true, platform_admin: false, teacher: false, teacher_application: false },
    courses: [],
    delegated_permissions: [],
    organizations: [{
      ...emptyScope,
      capabilities: [{ enabled: true, key: "settings", label: "Settings", permissions: ["MANAGE_ORG_SETTINGS"] }],
      direct_permissions: ["MANAGE_ORG_SETTINGS"],
      effective_permissions: ["MANAGE_ORG_SETTINGS"],
      id: 4,
      name: "Ferris Org",
      roles: ["ADMIN"],
    }],
    platform: emptyScope,
    user: { email: "operator@example.com", email_verified: true, id: 2, kyc_verified: true, name: "Org Operator" },
  };
}

function detailFixture(): OrganizationDetail {
  return {
    id: 4,
    name: "Ferris Org",
    profile_url: null,
    website_link: "https://ferris.example",
  };
}
