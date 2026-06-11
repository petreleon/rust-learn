"use client";

import { type TeacherApplicationStatus } from "@/lib/admin";

export function statusTone(status: TeacherApplicationStatus): "good" | "neutral" | "warn" {
  if (status === "approved") {
    return "good";
  }
  if (status === "submitted" || status === "needs_changes") {
    return "warn";
  }
  return "neutral";
}
