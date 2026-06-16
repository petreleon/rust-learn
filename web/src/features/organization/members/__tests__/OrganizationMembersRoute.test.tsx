import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type OrganizationMemberList } from "@/lib/organization/OrganizationMemberList";
import { OrganizationMembersRoute } from "../route/OrganizationMembersRoute";
import {
  addMemberByEmail,
  loadCurrentOrganizationSession,
  loadOrganizationMembers,
} from "../api/memberApi";

vi.mock("next/navigation", () => ({
  usePathname: () => "/organizations/4/members",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));
vi.mock("../api/memberApi", () => ({
  addMemberByEmail: vi.fn(),
  assignMemberRole: vi.fn(),
  loadCurrentOrganizationSession: vi.fn(),
  loadOrganizationMembers: vi.fn(),
  removeMemberFromOrganization: vi.fn(),
}));

const emptyScope = { capabilities: [], delegated_permissions: [], direct_permissions: [], effective_permissions: [], roles: [] };

describe("OrganizationMembersRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("org-token");
    vi.mocked(loadCurrentOrganizationSession).mockResolvedValue(sessionFixture());
    vi.mocked(loadOrganizationMembers).mockResolvedValue(memberListFixture());
    vi.mocked(addMemberByEmail).mockResolvedValue({ added: true });
  });

  it("shows signed-out state without loading the workflow", async () => {
    vi.mocked(readBrowserSessionToken).mockReturnValue(null);

    render(<OrganizationMembersRoute organizationId="4" />);

    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
    expect(loadCurrentOrganizationSession).not.toHaveBeenCalled();
    expect(loadOrganizationMembers).not.toHaveBeenCalled();
  });

  it("loads organization members through the feature API", async () => {
    render(<OrganizationMembersRoute organizationId="4" />);

    expect(await screen.findByRole("heading", { name: "Member directory" })).toBeVisible();
    expect(screen.getByText("Ada Operator")).toBeVisible();
    expect(loadOrganizationMembers).toHaveBeenCalledWith({
      limit: 8,
      offset: 0,
      organizationId: 4,
      permission: "",
      role: "",
      search: "",
      token: "org-token",
    });
  });

  it("submits member invites through the controller", async () => {
    const user = userEvent.setup();
    render(<OrganizationMembersRoute organizationId="4" />);

    await screen.findByRole("heading", { name: "Member directory" });
    await user.type(screen.getByPlaceholderText("user@example.com"), " new.user@example.com ");
    await user.click(screen.getByRole("button", { name: "Add member" }));

    await waitFor(() => expect(addMemberByEmail).toHaveBeenCalled());
    expect(addMemberByEmail).toHaveBeenCalledWith({
      email: "new.user@example.com",
      organizationId: 4,
      roleName: undefined,
      token: "org-token",
    });
    expect(await screen.findByText("Member added.")).toBeVisible();
  });
});

function sessionFixture(): CurrentSession {
  return {
    access: { learner: true, organization: true, platform_admin: false, teacher: false, teacher_application: false },
    courses: [],
    delegated_permissions: [],
    organizations: [{
      ...emptyScope,
      capabilities: [{ enabled: true, key: "members", label: "Members", permissions: ["VIEW_ORGANIZATION"] }],
      direct_permissions: ["VIEW_ORGANIZATION"],
      effective_permissions: ["VIEW_ORGANIZATION"],
      id: 4,
      name: "Ferris Org",
      roles: ["ADMIN"],
    }],
    platform: emptyScope,
    user: { email: "operator@example.com", email_verified: true, id: 2, kyc_verified: true, name: "Org Operator" },
  };
}

function memberListFixture(): OrganizationMemberList {
  return {
    limit: 8,
    members: [{
      delegated_permission_count: 0,
      delegated_permissions: [],
      direct_permission_count: 1,
      direct_permissions: ["VIEW_ORGANIZATION"],
      effective_permission_count: 1,
      effective_permissions: ["VIEW_ORGANIZATION"],
      email: "ada@example.com",
      email_verified: true,
      id: 7,
      joined_at: "2026-06-01T10:00:00Z",
      kyc_verified: true,
      name: "Ada Operator",
      roles: ["ADMIN"],
    }],
    offset: 0,
    operator_permissions: {
      can_assign_roles: true,
      can_invite_members: true,
      can_manage_members: true,
      can_manage_settings: true,
      can_view_members: true,
    },
    organization: { id: 4, name: "Ferris Org" },
    permission: null,
    role: null,
    search: null,
    total: 1,
  };
}
