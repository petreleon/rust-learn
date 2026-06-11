"use client";

import { type TeacherApplicationScope } from "@/lib/teacher";

export function isTeacherScope(value: unknown): value is TeacherApplicationScope {
  return value === "platform" || value === "organization" || value === "course";
}
