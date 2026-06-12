import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ProductShell } from "@/components/product-shell";
import { type CurrentSession } from "@/lib/session";

const navigationMock = vi.hoisted(() => ({
  pathname: "/session",
  push: vi.fn(),
}));
const emptyScope = { delegated_permissions: [], direct_permissions: [], effective_permissions: [], roles: [] };

vi.mock("next/navigation", () => ({
  usePathname: () => navigationMock.pathname,
  useRouter: () => ({ push: navigationMock.push }),
}));

function makeSession({
  platformPermissions = [],
  workspace = false,
}: {
  platformPermissions?: string[];
  workspace?: boolean;
} = {}): CurrentSession {
  return {
    courses: workspace
      ? [{ ...emptyScope, id: 3, lifecycle_status: "published", title: "Rust 101" }]
      : [],
    delegated_permissions: [],
    organizations: workspace
      ? [{ ...emptyScope, effective_permissions: ["VIEW_ORGANIZATION"], id: 2, name: "ACME" }]
      : [],
    platform: {
      delegated_permissions: [],
      direct_permissions: platformPermissions,
      effective_permissions: platformPermissions,
      roles: platformPermissions.length ? ["platform_admin"] : [],
    },
    user: {
      email: "learner@example.com",
      email_verified: true,
      id: 7,
      kyc_verified: false,
      name: "Learner User",
    },
  };
}

function renderShell({
  activeNav = "session",
  isSignedIn,
  session = makeSession(),
}: {
  activeNav?: "session" | "learn" | "teach" | "organizations" | "admin" | "account" | "none";
  isSignedIn?: boolean;
  session?: CurrentSession | null;
} = {}) {
  render(
    <ProductShell
      activeNav={activeNav}
      description="Profile and workspace routes."
      eyebrow="Workspace"
      isSignedIn={isSignedIn ?? Boolean(session)}
      session={session}
      title="Current session"
    >
      <p>Shell body</p>
    </ProductShell>,
  );
}

describe("ProductShell rendered navigation", () => {
  beforeEach(() => {
    navigationMock.pathname = "/session";
    navigationMock.push.mockClear();
  });

  it("keeps operations links out of normal user account menus", () => {
    renderShell();

    expect(screen.queryByText("Operations console")).not.toBeInTheDocument();
    expect(screen.queryByRole("link", { name: "Admin" })).not.toBeInTheDocument();
    expect(screen.getAllByText("Learner User").length).toBeGreaterThan(0);
  });

  it("exposes the operations console only for platform admins", () => {
    renderShell({ session: makeSession({ platformPermissions: ["MANAGE_ROLE_PERMISSIONS"] }) });

    const operationLinks = screen.getAllByText("Operations console").map((node) => node.closest("a"));
    const adminLinks = screen.getAllByRole("link", { name: "Admin" });

    expect(adminLinks).toHaveLength(2);
    adminLinks.forEach((link) => expect(link).toHaveAttribute("href", "/admin"));
    expect(operationLinks).toHaveLength(2);
    operationLinks.forEach((link) => expect(link).toHaveAttribute("href", "/ops"));
  });

  it("marks the active primary nav item on desktop and mobile surfaces", () => {
    renderShell({ activeNav: "learn" });

    const learnLinks = screen.getAllByRole("link", { name: "Learn" });

    expect(learnLinks).toHaveLength(2);
    learnLinks.forEach((link) => expect(link).toHaveAttribute("aria-current", "page"));
  });

  it("disables signed-out workspace and notification controls", () => {
    renderShell({ isSignedIn: false, session: null });

    expect(screen.getByRole("combobox", { name: /^Workspace$/ })).toBeDisabled();
    expect(screen.getByRole("combobox", { name: "Mobile workspace" })).toBeDisabled();
    expect(screen.getAllByRole("button", { name: "Notifications" }).every((button) => button.hasAttribute("disabled"))).toBe(true);
    expect(screen.getAllByRole("link", { name: "Sign in" })).toHaveLength(2);
  });

  it("routes desktop and mobile workspace selector changes", async () => {
    const user = userEvent.setup();
    renderShell({ session: makeSession({ workspace: true }) });

    await user.selectOptions(screen.getByRole("combobox", { name: /^Workspace$/ }), "organization:2");
    await user.selectOptions(screen.getByRole("combobox", { name: "Mobile workspace" }), "course:3");

    expect(navigationMock.push).toHaveBeenNthCalledWith(1, "/organizations/2");
    expect(navigationMock.push).toHaveBeenNthCalledWith(2, "/courses/3");
  });

  it("opens the mobile menu without losing workspace controls", async () => {
    const user = userEvent.setup();
    renderShell({ session: makeSession({ workspace: true }) });

    const menuSummary = screen.getByText("Menu");
    const menu = menuSummary.closest("details");
    expect(menu).not.toHaveAttribute("open");

    await user.click(menuSummary);

    expect(menu).toHaveAttribute("open");
    expect(screen.getByRole("combobox", { name: "Mobile workspace" })).toBeEnabled();
  });
});
