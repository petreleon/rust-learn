"use client";

import { type TeacherApplicationStatus } from "@/lib/admin";

export function isFinalApplicationStatus(status: TeacherApplicationStatus) {
  return status === "approved" || status === "rejected";
}
