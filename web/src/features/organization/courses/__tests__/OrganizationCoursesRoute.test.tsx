import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readStoredSessionToken } from "@/lib/session";
import { useOrganizationSession } from "@/features/organization/shared/route-kit/useOrganizationSession";
import { OrganizationCoursesRoute } from "../route/OrganizationCoursesRoute";
import {
  createDraftOrganizationCourse,
  loadOrganizationCourses,
  saveOrganizationCourseLifecycle,
  saveOrganizationCourseSettings,
} from "../api/courseApi";
import { courseListFixture, sessionFixture } from "./organizationCoursesTestFixtures";

vi.mock("next/navigation", () => ({
  usePathname: () => "/organizations/4/courses",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/lib/session", async () => {
  const actual = await vi.importActual<typeof import("@/lib/session")>("@/lib/session");
  return {
    ...actual,
    clearStoredSessionToken: vi.fn(),
    readStoredSessionToken: vi.fn(),
  };
});

vi.mock("@/features/organization/shared/route-kit/useOrganizationSession", () => ({
  useOrganizationSession: vi.fn(),
}));

vi.mock("../api/courseApi", () => ({
  createDraftOrganizationCourse: vi.fn(),
  loadOrganizationCourses: vi.fn(),
  saveOrganizationCourseLifecycle: vi.fn(),
  saveOrganizationCourseSettings: vi.fn(),
}));

describe("OrganizationCoursesRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readStoredSessionToken).mockReturnValue("org-token");
    vi.mocked(useOrganizationSession).mockReturnValue({
      error: null,
      hasToken: true,
      loadSession: vi.fn(),
      loadState: "success",
      session: sessionFixture(),
      signOut: vi.fn(),
    });
    vi.mocked(loadOrganizationCourses).mockResolvedValue(courseListFixture());
    vi.mocked(createDraftOrganizationCourse).mockResolvedValue({
      description: null,
      id: 41,
      lifecycle_status: "draft",
      prerequisites: null,
      title: "Org Rust Lab",
      topics: null,
    });
    vi.mocked(saveOrganizationCourseSettings).mockResolvedValue({
      description: null,
      id: 9,
      lifecycle_status: "published",
      prerequisites: null,
      title: "Renamed Org Lab",
      topics: null,
    });
    vi.mocked(saveOrganizationCourseLifecycle).mockResolvedValue({
      description: null,
      id: 9,
      lifecycle_status: "archived",
      prerequisites: null,
      title: "Rust Ownership Lab",
      topics: null,
    });
  });

  it("loads organization courses through the feature API", async () => {
    render(<OrganizationCoursesRoute organizationId="4" />);

    expect(await screen.findByRole("heading", { name: "Course list" })).toBeVisible();
    expect(screen.getAllByText("Rust Ownership Lab").length).toBeGreaterThan(0);
    expect(loadOrganizationCourses).toHaveBeenCalledWith({
      lifecycleStatus: "",
      limit: 6,
      offset: 0,
      organizationId: 4,
      rewardAvailable: null,
      search: "",
      token: "org-token",
    });
  });

  it("creates a draft organization course through the feature API", async () => {
    const user = userEvent.setup();
    render(<OrganizationCoursesRoute organizationId="4" />);

    await user.type(await screen.findByPlaceholderText("Ferris Org course title"), "Org Rust Lab");
    await user.click(screen.getByRole("button", { name: "Create draft course" }));

    await waitFor(() =>
      expect(createDraftOrganizationCourse).toHaveBeenCalledWith({
        organizationId: 4,
        title: "Org Rust Lab",
        token: "org-token",
      }),
    );
    expect(await screen.findByText("Org Rust Lab was created as draft.")).toBeVisible();
    expect(loadOrganizationCourses).toHaveBeenCalledTimes(2);
  });

  it("updates selected organization course settings and lifecycle", async () => {
    const user = userEvent.setup();
    render(<OrganizationCoursesRoute organizationId="4" />);

    await user.selectOptions(await screen.findByLabelText("Course to manage"), "9");
    const managementPanel = (await screen.findByRole("heading", { name: "Manage course" })).closest("section") as HTMLElement;
    const titleInput = within(managementPanel).getByLabelText("Title");
    await user.clear(titleInput);
    await user.type(titleInput, "Renamed Org Lab");
    await user.click(screen.getByRole("button", { name: "Save course settings" }));

    await waitFor(() =>
      expect(saveOrganizationCourseSettings).toHaveBeenCalledWith({
        courseId: 9,
        draft: { title: "Renamed Org Lab" },
        token: "org-token",
      }),
    );
    expect(await screen.findByText("Course settings updated.")).toBeVisible();

    await user.selectOptions(screen.getByLabelText("Lifecycle target"), "archived");
    await user.click(screen.getByRole("button", { name: "Update lifecycle" }));

    await waitFor(() =>
      expect(saveOrganizationCourseLifecycle).toHaveBeenCalledWith({
        courseId: 9,
        status: "archived",
        token: "org-token",
      }),
    );
    expect(await screen.findByText("Course lifecycle updated.")).toBeVisible();
  });
});
