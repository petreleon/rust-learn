import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type OrganizationDetail } from "@/lib/organization/OrganizationDetail";
import { OrganizationRequestError } from "@/lib/organization/OrganizationRequestError";
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

  it("shows a missing-organization state for unseen organization ids", async () => {
    vi.mocked(loadCurrentOrganizationSession).mockResolvedValue(missingOrganizationSession());

    render(<OrganizationSettingsRoute organizationId="4" />);

    expect(await screen.findByRole("heading", { name: "Organization unavailable" })).toBeVisible();
    expect(loadOrganizationSettings).not.toHaveBeenCalled();
  });

  it("shows a permission-denied state without settings permission", async () => {
    vi.mocked(loadCurrentOrganizationSession).mockResolvedValue(viewOnlySession());

    render(<OrganizationSettingsRoute organizationId="4" />);

    expect(await screen.findByRole("heading", { name: "Settings unavailable" })).toBeVisible();
    expect(screen.getByText(/MANAGE_ORG_SETTINGS/)).toBeVisible();
    expect(loadOrganizationSettings).not.toHaveBeenCalled();
  });

  it("shows a backend-error state when settings cannot load", async () => {
    vi.mocked(loadOrganizationSettings).mockRejectedValue(new Error("network down"));

    render(<OrganizationSettingsRoute organizationId="4" />);

    expect((await screen.findAllByText("Organization settings could not be loaded.")).length).toBeGreaterThan(0);
    expect(screen.getByRole("button", { name: "Retry" })).toBeVisible();
  });

  it("shows a timeout state when settings cannot finish loading", async () => {
    vi.mocked(loadOrganizationSettings).mockRejectedValue(
      new OrganizationRequestError("Organization request timed out.", 0, "timeout"),
    );

    render(<OrganizationSettingsRoute organizationId="4" />);

    expect((await screen.findAllByText("Organization request timed out.")).length).toBeGreaterThan(0);
    expect(screen.getByRole("button", { name: "Retry" })).toBeVisible();
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

function missingOrganizationSession(): CurrentSession {
  return { ...sessionFixture(), access: { ...sessionFixture().access, organization: false }, organizations: [] };
}

function viewOnlySession(): CurrentSession {
  const organization = sessionFixture().organizations[0];
  return {
    ...sessionFixture(),
    organizations: [{
      ...organization,
      capabilities: [{ enabled: false, key: "settings", label: "Settings", permissions: ["MANAGE_ORG_SETTINGS"] }],
      direct_permissions: ["VIEW_ORGANIZATION"],
      effective_permissions: ["VIEW_ORGANIZATION"],
      roles: ["VIEWER"],
    }],
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
