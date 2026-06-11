"use client";

import { type PermissionOption } from "./PermissionOption";

export const PERMISSIONS: PermissionOption[] = [
  { key: "SUBMIT_TEACHER_APPLICATION", label: "Submit application", scope: "platform" },
  { key: "REVIEW_TEACHER_APPLICATIONS", label: "Review applications", scope: "platform" },
  { key: "APPROVE_TEACHER_APPLICATION", label: "Approve teachers", scope: "platform" },
  { key: "REJECT_TEACHER_APPLICATION", label: "Reject teachers", scope: "platform" },
  { key: "SUBMIT_COURSE_REWARD_EVENT", label: "Submit course reward", scope: "course" },
  { key: "VIEW_COURSE_REWARD_STATUS", label: "View course rewards", scope: "course" },
  {
    key: "APPROVE_STUDENT_REWARD_CANDIDATE",
    label: "Approve student reward",
    scope: "course",
  },
  { key: "APPROVE_REWARD_AMOUNT", label: "Approve amount", scope: "platform" },
  { key: "VIEW_REPORT", label: "View summary reports", scope: "platform" },
  { key: "GENERATE_REPORT", label: "Generate summary CSV", scope: "organization" },
  { key: "VIEW_ORG_REWARD_REPORTS", label: "View org rewards", scope: "organization" },
  { key: "VIEW_REWARD_AUDIT", label: "View reward audit", scope: "platform" },
  { key: "MANAGE_REWARD_FRAUD_BLOCKS", label: "Manage fraud blocks", scope: "platform" },
  { key: "BLOCK_REWARD_TEACHER", label: "Block teacher rewards", scope: "platform" },
  { key: "BLOCK_REWARD_ORGANIZATION", label: "Block org rewards", scope: "platform" },
  { key: "DELEGATE_REWARD_APPROVAL", label: "Delegate reward approval", scope: "platform" },
  { key: "EXPORT_DATA", label: "Export platform data", scope: "platform" },
];
