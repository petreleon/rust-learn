"use client";

import { type OrganizationCapabilityKey } from "@/lib/organization";

export const actionDescriptions: Record<OrganizationCapabilityKey, string> = {
  courses: "Inspect sponsored courses, lifecycle, teacher coverage, enrollment pressure, and reward policy status.",
  course_rewards: "Submit organization-backed course reward events when the route contract is added.",
  member_management: "Invite members, assign organization roles, and inspect management readiness.",
  members: "Inspect organization members, role labels, scoped permissions, and management readiness.",
  reports: "Inspect reward volume, sponsored applications, wallet balances, and CSV exports.",
  settings: "Manage organization name, website, profile URL, and destructive actions scoped to operator permissions.",
  teacher_applications: "Track sponsored teacher applications, decisions, applicant context, and audit hints.",
  wallet: "Review wallet balance, budget readiness, reward credits, token links, and audit rows.",
};
