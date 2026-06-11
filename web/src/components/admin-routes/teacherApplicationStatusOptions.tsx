"use client";

import { type TeacherApplicationStatus } from "@/lib/admin";

export const teacherApplicationStatusOptions: Array<{ label: string; value: TeacherApplicationStatus | "" }> = [
  { label: "All statuses", value: "" },
  { label: "Submitted", value: "submitted" },
  { label: "Needs changes", value: "needs_changes" },
  { label: "Approved", value: "approved" },
  { label: "Rejected", value: "rejected" },
];
