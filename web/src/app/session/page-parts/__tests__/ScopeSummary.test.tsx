import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { ScopeSummary } from "../ScopeSummary";
import { type PlatformSessionScope } from "@/lib/session";

function scope(overrides: Partial<PlatformSessionScope> = {}): PlatformSessionScope {
  return {
    delegated_permissions: [],
    direct_permissions: [],
    effective_permissions: [],
    roles: [],
    ...overrides,
  };
}

describe("ScopeSummary", () => {
  it("summarizes large permission sets before showing raw keys", async () => {
    const user = userEvent.setup();

    render(
      <ScopeSummary
        scope={scope({
          delegated_permissions: ["VIEW_REWARD_AUDIT"],
          direct_permissions: ["APPROVE_COURSE_CONTENT"],
          effective_permissions: [
            "APPROVE_COURSE_CONTENT",
            "MANAGE_REWARD_FRAUD_BLOCKS",
            "VIEW_REWARD_AUDIT",
            "EXPORT_DATA",
          ],
          roles: ["SUPER_ADMIN"],
        })}
        title="Platform"
      />,
    );

    expect(screen.getByText("Super admin")).toBeVisible();
    expect(screen.getByText("4 permissions")).toBeVisible();
    expect(screen.getByText("1 direct")).toBeVisible();
    expect(screen.getByText("1 delegated")).toBeVisible();
    expect(screen.getByText("APPROVE_COURSE_CONTENT")).not.toBeVisible();

    await user.click(screen.getByText("View permission details"));

    expect(screen.getByText("Approve course content")).toBeVisible();
    expect(screen.getByText("APPROVE_COURSE_CONTENT")).toBeVisible();
  });

  it("shows an empty access state without a details disclosure", () => {
    render(<ScopeSummary scope={scope()} title="Access" />);

    expect(screen.getByText("No role")).toBeVisible();
    expect(screen.getByText("No permissions")).toBeVisible();
    expect(screen.queryByText("View permission details")).not.toBeInTheDocument();
  });

  it("labels delegated-only access without inventing a role", () => {
    render(
      <ScopeSummary
        scope={scope({
          delegated_permissions: ["VIEW_REPORT"],
          effective_permissions: ["VIEW_REPORT"],
        })}
        title="Course"
      />,
    );

    expect(screen.getByText("Delegated access")).toBeVisible();
    expect(screen.getByText("1 permission")).toBeVisible();
    expect(screen.getByText("1 delegated")).toBeVisible();
  });
});
