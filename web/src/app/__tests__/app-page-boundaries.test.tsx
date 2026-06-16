import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import AdminFraudBlocksPage from "@/app/admin/fraud-blocks/page";
import AdminRewardPoliciesPage from "@/app/admin/reward-policies/page";
import AdminUsersPage from "@/app/admin/users/page";
import AdminWalletsPage from "@/app/admin/wallets/page";
import CourseLearnPage from "@/app/courses/[courseId]/learn/page";
import ForgotPasswordPage from "@/app/forgot-password/page";
import LearnerWorkspacePage from "@/app/learn/page";
import LoginPage from "@/app/login/page";
import OrganizationCoursesPage from "@/app/organizations/[organizationId]/courses/page";
import OrganizationWorkspacePage from "@/app/organizations/page";
import RegisterPage from "@/app/register/page";
import ResetPasswordPage from "@/app/reset-password/page";
import AccountSettingsPage from "@/app/settings/account/page";
import SessionPage from "@/app/session/page";
import TeachingCourseContentPage from "@/app/teach/courses/[courseId]/content/page";
import TeacherWorkspacePage from "@/app/teach/page";
import VerifyEmailPage from "@/app/verify-email/page";
import WalletPage from "@/app/wallet/page";

vi.mock("@/features/auth/login/route/LoginRoute", () => ({ default: () => <div data-testid="login-route" /> }));
vi.mock("@/features/auth/register/route/RegisterRoute", () => ({ default: () => <div data-testid="register-route" /> }));
vi.mock("@/features/auth/forgot-password/route/ForgotPasswordRoute", () => ({
  default: () => <div data-testid="forgot-password-route" />,
}));
vi.mock("@/features/auth/reset-password/route/ResetPasswordRoute", () => ({
  default: () => <div data-testid="reset-password-route" />,
}));
vi.mock("@/features/auth/verify-email/route/VerifyEmailRoute", () => ({
  default: () => <div data-testid="verify-email-route" />,
}));
vi.mock("@/features/session/workspace/route/SessionWorkspaceRoute", () => ({
  SessionWorkspaceRoute: () => <div data-testid="session-route" />,
}));
vi.mock("@/features/session/account-settings/route/AccountSettingsRoute", () => ({
  AccountSettingsRoute: () => <div data-testid="account-settings-route" />,
}));
vi.mock("@/features/learner/workspace/components/LearnerDashboardRoute", () => ({
  LearnerDashboardRoute: () => <div data-testid="learner-dashboard-route" />,
}));
vi.mock("@/features/learner/workspace/components/LearnerProductRoute", () => ({
  LearnerProductRoute: ({ kind }: { kind: string }) => <div data-testid={`learner-product-${kind}`} />,
}));
vi.mock("@/features/learner/workspace/components/LearnerCourseLearnRoute", () => ({
  LearnerCourseLearnRoute: ({ courseId }: { courseId: string }) => <div data-testid="course-learn-route">course:{courseId}</div>,
}));
vi.mock("@/features/organization/index/route/OrganizationIndexRoute", () => ({
  OrganizationIndexRoute: () => <div data-testid="organization-index-route" />,
}));
vi.mock("@/features/organization/courses/route/OrganizationCoursesRoute", () => ({
  OrganizationCoursesRoute: ({ organizationId }: { organizationId: string }) => (
    <div data-testid="organization-courses-route">organization:{organizationId}</div>
  ),
}));
vi.mock("@/features/teacher/teaching-workspace/route/TeachingWorkspaceRoute", () => ({
  TeachingWorkspaceRoute: ({ view }: { view: string }) => <div data-testid="teaching-workspace-route">view:{view}</div>,
}));
vi.mock("@/features/teacher/course-content/route/TeacherCourseContentRoute", () => ({
  TeacherCourseContentRoute: ({ courseId }: { courseId: string }) => <div data-testid="teacher-content-route">course:{courseId}</div>,
}));
vi.mock("@/features/admin/users/route/AdminUsersRoute", () => ({
  AdminUsersRoute: () => <div data-testid="admin-users-route" />,
}));
vi.mock("@/features/admin/reward-policies/route/AdminRewardPoliciesRoute", () => ({
  AdminRewardPoliciesRoute: () => <div data-testid="admin-reward-policies-route" />,
}));
vi.mock("@/features/admin/fraud-blocks/route/AdminFraudBlocksRoute", () => ({
  AdminFraudBlocksRoute: () => <div data-testid="admin-fraud-blocks-route" />,
}));
vi.mock("@/features/admin/wallets/route/AdminWalletsRoute", () => ({
  AdminWalletsRoute: () => <div data-testid="admin-wallets-route" />,
}));

function renderPage(element: React.ReactElement, testId: string) {
  const result = render(element);
  expect(screen.getByTestId(testId)).toBeVisible();
  return result;
}

function expectPage(element: React.ReactElement, testId: string) {
  renderPage(element, testId).unmount();
}

describe("app page feature boundaries", () => {
  it("mounts auth, session, account, and product page adapters", () => {
    expectPage(<LoginPage />, "login-route");
    expectPage(<RegisterPage />, "register-route");
    expectPage(<ForgotPasswordPage />, "forgot-password-route");
    expectPage(<ResetPasswordPage />, "reset-password-route");
    expectPage(<VerifyEmailPage />, "verify-email-route");
    expectPage(<SessionPage />, "session-route");
    expectPage(<AccountSettingsPage />, "account-settings-route");
    expectPage(<LearnerWorkspacePage />, "learner-dashboard-route");
    expectPage(<WalletPage />, "learner-product-wallet");
    expectPage(<OrganizationWorkspacePage />, "organization-index-route");
    expectPage(<TeacherWorkspacePage />, "teaching-workspace-route");
    expectPage(<AdminUsersPage />, "admin-users-route");
    expectPage(<AdminRewardPoliciesPage />, "admin-reward-policies-route");
    expectPage(<AdminFraudBlocksPage />, "admin-fraud-blocks-route");
    expectPage(<AdminWalletsPage />, "admin-wallets-route");
  });

  it("passes dynamic app route params into feature routes", async () => {
    const courseLearn = renderPage(await CourseLearnPage({ params: Promise.resolve({ courseId: "42" }) }), "course-learn-route");
    expect(screen.getByText("course:42")).toBeVisible();
    courseLearn.unmount();

    const teacherContent = renderPage(await TeachingCourseContentPage({ params: Promise.resolve({ courseId: "9" }) }), "teacher-content-route");
    expect(screen.getByText("course:9")).toBeVisible();
    teacherContent.unmount();

    const organizationCourses = renderPage(
      await OrganizationCoursesPage({ params: Promise.resolve({ organizationId: "77" }) }),
      "organization-courses-route",
    );
    expect(screen.getByText("organization:77")).toBeVisible();
    organizationCourses.unmount();
  });
});
