"use client";

import { type EnrollmentStatusFilter } from "./EnrollmentStatusFilter";

export const enrollmentFilterOptions: Array<{ label: string; value: EnrollmentStatusFilter }> = [
  { label: "All", value: "all" },
  { label: "Available", value: "available" },
  { label: "Pending", value: "pending" },
  { label: "Waitlisted", value: "waitlisted" },
  { label: "Enrolled", value: "enrolled" },
  { label: "Rejected", value: "rejected" },
  { label: "Unavailable", value: "unavailable" },
];
