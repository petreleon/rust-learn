import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readStoredSessionToken } from "@/lib/session";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type OrganizationDetail } from "@/lib/organization";
import { OrganizationIndexRoute } from "../route/OrganizationIndexRoute";
import { loadOrganizationIndexSession, loadPlatformOrganizations } from "../api/organizationIndexApi";

vi.mock("next/navigation", () => ({
  usePathname: () => "/organizations",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/lib/session", () => ({
  clearStoredSessionToken: vi.fn(),
  readStoredSessionToken: vi.fn(),
}));

vi.mock("../api/organizationIndexApi", () => ({
  loadOrganizationIndexSession: vi.fn(),
  loadPlatformOrganizations: vi.fn(),
}));

describe("OrganizationIndexRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readStoredSessionToken).mockReturnValue("org-token");
    vi.mocked(loadOrganizationIndexSession).mockResolvedValue(platformAdminSession());
    vi.mocked(loadPlatformOrganizations).mockResolvedValue(platformOrganizations());
  });

  it("shows signed-out state without loading organizations", async () => {
    vi.mocked(readStoredSessionToken).mockReturnValue(null);

    render(<OrganizationIndexRoute />);

    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
    expect(loadOrganizationIndexSession).not.toHaveBeenCalled();
    expect(loadPlatformOrganizations).not.toHaveBeenCalled();
  });

  it("loads the platform organization directory for platform admins", async () => {
    render(<OrganizationIndexRoute />);

    expect(await screen.findByRole("heading", { name: "Platform directory" })).toBeVisible();
    expect(await screen.findByText("Ferris Foundation")).toBeVisible();
    expect(await screen.findByText("Oxide School")).toBeVisible();
    expect(loadOrganizationIndexSession).toHaveBeenCalledWith({ token: "org-token" });
    await waitFor(() => expect(loadPlatformOrganizations).toHaveBeenCalledWith({ token: "org-token" }));
    expect(screen.queryByText("No organization workspace yet")).not.toBeInTheDocument();
  });

  it("filters platform organizations by search", async () => {
    const user = userEvent.setup();

    render(<OrganizationIndexRoute />);

    await screen.findByText("Ferris Foundation");
    await user.type(screen.getByPlaceholderText("Name, id, or URL"), "oxide");

    expect(screen.getByText("Oxide School")).toBeVisible();
    expect(screen.queryByText("Ferris Foundation")).not.toBeInTheDocument();
  });

  it("keeps ordinary users with no organizations in the empty workspace state", async () => {
    vi.mocked(loadOrganizationIndexSession).mockResolvedValue(ordinarySession());

    render(<OrganizationIndexRoute />);

    expect(await screen.findByText("No organization workspace yet")).toBeVisible();
    expect(loadPlatformOrganizations).not.toHaveBeenCalled();
  });
});

function platformAdminSession(): CurrentSession {
  return {
    access: { learner: true, organization: false, platform_admin: true, teacher: false, teacher_application: false },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      capabilities: [{ enabled: true, key: "summary", label: "Platform summary", permissions: ["VIEW_ORGANIZATION"] }],
      delegated_permissions: [],
      direct_permissions: ["VIEW_ORGANIZATION"],
      effective_permissions: ["VIEW_ORGANIZATION"],
      roles: ["platform_admin"],
    },
    user: { email: "admin@example.com", email_verified: true, id: 1, kyc_verified: true, name: "Admin User" },
  };
}

function ordinarySession(): CurrentSession {
  return {
    ...platformAdminSession(),
    access: { learner: true, organization: false, platform_admin: false, teacher: false, teacher_application: false },
    platform: {
      capabilities: [],
      delegated_permissions: [],
      direct_permissions: [],
      effective_permissions: [],
      roles: [],
    },
  };
}

function platformOrganizations(): OrganizationDetail[] {
  return [
    {
      id: 4,
      name: "Ferris Foundation",
      profile_url: null,
      website_link: "https://ferris.example",
    },
    {
      id: 9,
      name: "Oxide School",
      profile_url: "https://oxide.example/profile",
      website_link: null,
    },
  ];
}
