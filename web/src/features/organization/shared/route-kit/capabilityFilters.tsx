"use client";

import { type CapabilityFilter } from "./CapabilityFilter";

export const capabilityFilters: Array<{ label: string; value: CapabilityFilter }> = [
  { label: "All access", value: "all" },
  { label: "Courses", value: "courses" },
  { label: "Reports", value: "reports" },
  { label: "Members", value: "members" },
  { label: "Wallet", value: "wallet" },
  { label: "Teacher nominations", value: "teacher_applications" },
  { label: "Course rewards", value: "course_rewards" },
  { label: "Settings", value: "settings" },
  { label: "Delegated", value: "delegated" },
];
