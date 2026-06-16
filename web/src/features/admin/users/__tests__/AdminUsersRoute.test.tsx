import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { type AdminRole, type AdminUserProfile } from "@/lib/admin";
import { AdminRequestError } from "@/lib/admin";
import { type CurrentSession } from "@/lib/session";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  assignRoleToAdminUser,
  loadAdminPlatformRoles,
  loadAdminUserProfile,
  loadAdminUsersSession,
  searchAdminUsers,
} from "../api/adminUsersApi";
import { AdminUsersRoute } from "../route/AdminUsersRoute";

vi.mock("next/navigation", () => ({
  usePathname: () => "/admin/users",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/adminUsersApi", () => ({
  assignRoleToAdminUser: vi.fn(),
  loadAdminPlatformRoles: vi.fn(),
  loadAdminUserProfile: vi.fn(),
  loadAdminUsersSession: vi.fn(),
  searchAdminUsers: vi.fn(),
}));

describe("AdminUsersRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("admin-token");
    vi.mocked(loadAdminUsersSession).mockResolvedValue(adminSession(["VIEW_USER", "ASSIGN_ROLES_TO_USER", "VIEW_ROLE_ASSIGNMENTS"]));
    vi.mocked(loadAdminPlatformRoles).mockResolvedValue([role("PLATFORM_ADMIN")]);
    vi.mocked(searchAdminUsers).mockResolvedValue({ users: [profile(7)] });
    vi.mocked(loadAdminUserProfile).mockResolvedValue(
      profile(7, {
        kyc_verified: true,
        platform_permissions: ["VIEW_USER", "ASSIGN_ROLES_TO_USER"],
        platform_roles: ["LEARNER"],
      }),
    );
    vi.mocked(assignRoleToAdminUser).mockResolvedValue("Role assigned successfully");
  });

  it("shows signed-out state without loading admin users", async () => {
    vi.mocked(readBrowserSessionToken).mockReturnValue(null);

    render(<AdminUsersRoute />);

    expect(await screen.findByText("Sign in required")).toBeVisible();
    expect(loadAdminUsersSession).not.toHaveBeenCalled();
    expect(searchAdminUsers).not.toHaveBeenCalled();
  });

  it("searches users, loads selected profile, and assigns a platform role", async () => {
    const user = userEvent.setup();
    render(<AdminUsersRoute />);

    expect(await screen.findByText("User management")).toBeVisible();
    await waitFor(() => expect(loadAdminPlatformRoles).toHaveBeenCalledWith({ token: "admin-token" }));

    await user.type(screen.getByLabelText("Search users"), "ada");
    await user.click(screen.getByRole("button", { name: "Search" }));

    await waitFor(() => expect(searchAdminUsers).toHaveBeenCalledWith({ search: "ada", token: "admin-token" }));
    await user.click(await screen.findByRole("button", { name: /Ada Lovelace/ }));

    await waitFor(() => expect(loadAdminUserProfile).toHaveBeenCalledWith({ token: "admin-token", userId: 7 }));
    expect(await screen.findByText("KYC status")).toBeVisible();
    expect(await screen.findByText("Platform roles")).toBeVisible();
    expect(await screen.findByText("ASSIGN ROLES TO USER")).toBeVisible();
    await user.selectOptions(screen.getByLabelText("Role"), "PLATFORM_ADMIN");
    await user.click(screen.getByRole("button", { name: "Assign role" }));

    await waitFor(() =>
      expect(assignRoleToAdminUser).toHaveBeenCalledWith({
        roleName: "PLATFORM_ADMIN",
        token: "admin-token",
        userId: 7,
      }),
    );
    expect(await screen.findByText("Role assigned successfully.")).toBeVisible();
  });

  it("shows a gated panel when platform admin lacks user view permission", async () => {
    vi.mocked(loadAdminUsersSession).mockResolvedValue(adminSession(["EXPORT_DATA"]));

    render(<AdminUsersRoute />);

    expect(await screen.findByText("User management unavailable")).toBeVisible();
    expect(searchAdminUsers).not.toHaveBeenCalled();
  });

  it("supports manual role entry when the role catalog is gated", async () => {
    const user = userEvent.setup();
    vi.mocked(loadAdminUsersSession).mockResolvedValue(adminSession(["VIEW_USER", "ASSIGN_ROLES_TO_USER"]));

    render(<AdminUsersRoute />);

    await user.type(await screen.findByLabelText("Search users"), "ada");
    await user.click(screen.getByRole("button", { name: "Search" }));
    await user.click(await screen.findByRole("button", { name: /Ada Lovelace/ }));
    await user.type(await screen.findByPlaceholderText("Platform role name"), "SUPPORT_ADMIN");
    await user.click(screen.getByRole("button", { name: "Assign role" }));

    expect(loadAdminPlatformRoles).not.toHaveBeenCalled();
    await waitFor(() => expect(assignRoleToAdminUser).toHaveBeenCalledWith(expect.objectContaining({ roleName: "SUPPORT_ADMIN" })));
  });

  it("shows assignment backend errors without leaving the profile", async () => {
    const user = userEvent.setup();
    vi.mocked(assignRoleToAdminUser).mockRejectedValue(new AdminRequestError("Hierarchy check failed", 403, "hierarchy_denied"));

    render(<AdminUsersRoute />);

    await user.type(await screen.findByLabelText("Search users"), "ada");
    await user.click(screen.getByRole("button", { name: "Search" }));
    await user.click(await screen.findByRole("button", { name: /Ada Lovelace/ }));
    await user.click(await screen.findByRole("button", { name: "Assign role" }));

    expect(await screen.findByText("Hierarchy check failed")).toBeVisible();
    expect(screen.getAllByText("ada@example.com").length).toBeGreaterThan(0);
  });
});

function adminSession(permissions: string[]): CurrentSession {
  return {
    access: { learner: true, organization: false, platform_admin: permissions.length > 0, teacher: false, teacher_application: false },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      capabilities: [
        { enabled: permissions.includes("VIEW_USER"), key: "users", label: "User management", permissions: ["VIEW_USER", "ASSIGN_ROLES_TO_USER", "VIEW_ROLE_ASSIGNMENTS"] },
      ],
      delegated_permissions: [],
      direct_permissions: permissions,
      effective_permissions: permissions,
      roles: permissions.length ? ["platform_admin"] : [],
    },
    user: { email: "admin@example.com", email_verified: true, id: 1, kyc_verified: true, name: "Admin User" },
  };
}

function profile(id: number, overrides: Partial<AdminUserProfile> = {}): AdminUserProfile {
  return {
    created_at: "2026-06-12 10:00:00",
    date_of_birth: null,
    email: "ada@example.com",
    email_verified: true,
    id,
    kyc_verified: false,
    name: "Ada Lovelace",
    platform_permissions: [],
    platform_roles: [],
    ...overrides,
  };
}

function role(name: string): AdminRole {
  return { description: null, id: 3, name };
}
