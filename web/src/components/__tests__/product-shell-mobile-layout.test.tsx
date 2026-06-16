import { readFileSync } from "node:fs";
import { join } from "node:path";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ProductShell } from "@/components/product-shell";
import { type CurrentSession } from "@/lib/session";

const navigationMock = vi.hoisted(() => ({
  pathname: "/organizations/200",
  push: vi.fn(),
}));
const emptyScope = { capabilities: [], delegated_permissions: [], direct_permissions: [], effective_permissions: [], roles: [] };

vi.mock("next/navigation", () => ({
  usePathname: () => navigationMock.pathname,
  useRouter: () => ({ push: navigationMock.push }),
}));

function setViewport(width: number, height: number) {
  Object.defineProperty(window, "innerWidth", { configurable: true, value: width });
  Object.defineProperty(window, "innerHeight", { configurable: true, value: height });
  window.dispatchEvent(new Event("resize"));
}

function makeCrowdedSession(): CurrentSession {
  return {
    access: {
      learner: true,
      teacher: true,
      teacher_application: false,
      organization: true,
      platform_admin: true,
    },
    courses: [
      { ...emptyScope, id: 300, lifecycle_status: "published", title: "Extremely Long Rust Ownership Course" },
    ],
    delegated_permissions: [],
    organizations: [
      {
        ...emptyScope,
        effective_permissions: ["VIEW_ORGANIZATION"],
        id: 200,
        name: "Alexandria Distributed Systems Guild With Long Name",
      },
    ],
    platform: {
      delegated_permissions: [],
      direct_permissions: ["MANAGE_ROLE_PERMISSIONS"],
      effective_permissions: ["MANAGE_ROLE_PERMISSIONS"],
      capabilities: [
        { enabled: true, key: "delegations", label: "Delegations", permissions: ["MANAGE_ROLE_PERMISSIONS"] },
      ],
      roles: ["platform_admin"],
    },
    user: {
      email: "alexandria.operator.with.a.long.email@example.com",
      email_verified: true,
      id: 7,
      kyc_verified: false,
      name: "Alexandria Operator With A Very Long Display Name",
    },
  };
}

describe("ProductShell mobile layout coverage", () => {
  beforeEach(() => {
    navigationMock.pathname = "/organizations/200";
    navigationMock.push.mockClear();
    setViewport(390, 844);
  });

  it("keeps crowded mobile navigation and workspace controls available", async () => {
    const user = userEvent.setup();
    render(
      <ProductShell
        activeNav="organizations"
        breadcrumbs={[{ href: "/organizations", label: "Organizations" }, { label: "Long workspace branch" }]}
        description="Long workspace description that should remain readable when the phone viewport is narrow."
        eyebrow="Workspace"
        isSignedIn
        session={makeCrowdedSession()}
        statusItems={<span>Platform administrator with long status copy</span>}
        title="Organization workspace with long title"
      >
        <p>Shell body</p>
      </ProductShell>,
    );

    const summary = screen.getByText("Menu");
    const menu = summary.closest("details");
    await user.click(summary);

    expect(window.innerWidth).toBe(390);
    expect(menu).toHaveAttribute("open");
    expect(screen.getByRole("combobox", { name: "Mobile workspace" })).toHaveValue("organization:200");
    expect(screen.getAllByRole("link", { name: "Organizations" })).toHaveLength(3);
    expect(screen.getAllByText("Operations console")).toHaveLength(2);
    expect(screen.getAllByText("Alexandria Operator With A Very Long Display Name").length).toBeGreaterThan(0);
  });

  it("locks in ProductShell mobile CSS overflow guardrails", () => {
    const modulePath = join(process.cwd(), "src/components/product-shell.module");
    const css = ["01", "02", "03", "04", "05"]
      .map((part) => readFileSync(join(modulePath, `${part}.module.css`), "utf8"))
      .join("\n");

    expect(css).toMatch(/@media \(max-width: 1040px\)[\s\S]*\.desktopNav,\s*\.toolbar \{[\s\S]*display: none;/);
    expect(css).toMatch(/@media \(max-width: 1040px\)[\s\S]*\.mobileMenu \{[\s\S]*display: block;/);
    expect(css).toContain("width: min(320px, calc(100vw - 32px));");
    expect(css).toContain("width: min(360px, calc(100vw - 32px));");
    expect(css).toContain("min-width: 0;");
    expect(css).toContain("overflow-wrap: anywhere;");
    expect(css).toMatch(/\.statusStrip \{[\s\S]*flex-wrap: wrap;/);
  });
});
