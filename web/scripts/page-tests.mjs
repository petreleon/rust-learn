#!/usr/bin/env node

import { spawn } from "node:child_process";

const pageTestFiles = [
  "src/components/__tests__/product-shell-render.test.tsx",
  "src/features/admin/dashboard/__tests__/AdminDashboardRoute.test.tsx",
  "src/features/admin/delegations/__tests__/AdminDelegationsRoute.test.tsx",
  "src/features/admin/fraud-blocks/__tests__/AdminFraudBlocksRoute.test.tsx",
  "src/features/admin/reward-policies/__tests__/AdminRewardPoliciesRoute.test.tsx",
  "src/features/admin/reward-amount-review/__tests__/AdminRewardAmountReviewRoute.test.tsx",
  "src/features/admin/teacher-applications/__tests__/AdminTeacherApplicationsRoute.test.tsx",
  "src/features/admin/fraud-blocks/__tests__/AdminFraudBlockRewardPolicyRoute.test.tsx",
  "src/features/admin/users/__tests__/AdminUsersRoute.test.tsx",
  "src/features/admin/wallets/__tests__/AdminWalletsRoute.test.tsx",
  "src/features/learner/workspace/assessments/__tests__/CourseAssessmentEntryPanel.test.tsx",
  "src/features/learner/workspace/assessments/__tests__/CourseAssessmentPanel.test.tsx",
  "src/features/learner/workspace/wallet-transfers/__tests__/WalletTransferPanel.test.tsx",
  "src/features/organization/index/__tests__/OrganizationIndexRoute.test.tsx",
  "src/features/organization/courses/__tests__/OrganizationCoursesRoute.test.tsx",
  "src/features/organization/members/__tests__/OrganizationMembersRoute.test.tsx",
  "src/features/organization/settings/__tests__/OrganizationSettingsRoute.test.tsx",
  "src/features/session/account-settings/__tests__/AccountSettingsRoute.test.tsx",
  "src/features/session/workspace/__tests__/SessionWorkspaceRoute.test.tsx",
  "src/features/teacher/application/__tests__/TeacherApplicationRoute.test.tsx",
  "src/features/teacher/course-enrollments/__tests__/TeacherCourseEnrollmentsRoute.test.tsx",
  "src/components/__tests__/teacher-rewards-route.test.tsx",
  "src/features/teacher/course-rewards/__tests__/RewardPolicyCoverage.test.tsx",
  "src/features/teacher/course-workspace/__tests__/TeacherCourseWorkspaceRoute.test.tsx",
  "src/features/teacher/teaching-workspace/__tests__/TeachingWorkspaceRoute.test.tsx",
];

const command = process.platform === "win32" ? "vitest.cmd" : "vitest";
const child = spawn(command, ["run", ...pageTestFiles], {
  shell: process.platform === "win32",
  stdio: "inherit",
});

child.on("exit", (code) => {
  process.exit(code ?? 1);
});
