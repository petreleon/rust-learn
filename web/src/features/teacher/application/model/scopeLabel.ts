"use client";

import { type TeacherApplicationScope } from "@/lib/teacher";

export function scopeLabel(scope: TeacherApplicationScope) {
  if (scope === "platform") {
    return "Platform";
  }

  if (scope === "organization") {
    return "Organization";
  }

  return "Course";
}
