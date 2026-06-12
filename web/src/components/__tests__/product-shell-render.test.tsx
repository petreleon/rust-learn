import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ProductShell } from "@/components/product-shell";
import { type CurrentSession } from "@/lib/session";

vi.mock("next/navigation", () => ({
  usePathname: () => "/session",
  useRouter: () => ({ push: vi.fn() }),
}));

function makeSession(platformPermissions: string[] = []): CurrentSession {
  return {
    courses: [],
    delegated_permissions: [],
    organizations: [],
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

function renderShell(session: CurrentSession | null = makeSession()) {
  render(
    <ProductShell
      activeNav="session"
      description="Profile and workspace routes."
      eyebrow="Workspace"
      isSignedIn={Boolean(session)}
      session={session}
      title="Current session"
    >
      <p>Shell body</p>
    </ProductShell>,
  );
}

describe("ProductShell rendered navigation", () => {
  it("keeps operations links out of normal user account menus", () => {
    renderShell();

    expect(screen.queryByText("Operations console")).not.toBeInTheDocument();
    expect(screen.queryByRole("link", { name: "Admin" })).not.toBeInTheDocument();
    expect(screen.getAllByText("Learner User").length).toBeGreaterThan(0);
  });

  it("exposes the operations console only for platform admins", () => {
    renderShell(makeSession(["MANAGE_ROLE_PERMISSIONS"]));

    const operationLinks = screen.getAllByText("Operations console").map((node) => node.closest("a"));
    const adminLinks = screen.getAllByRole("link", { name: "Admin" });

    expect(adminLinks).toHaveLength(2);
    adminLinks.forEach((link) => expect(link).toHaveAttribute("href", "/admin"));
    expect(operationLinks).toHaveLength(2);
    operationLinks.forEach((link) => expect(link).toHaveAttribute("href", "/ops"));
  });
});
