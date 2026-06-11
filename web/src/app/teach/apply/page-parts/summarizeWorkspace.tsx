"use client";

import { type CurrentSession } from "@/lib/session";

export function summarizeWorkspace(session: CurrentSession | null) {
  if (!session) {
    return "No workspace is loaded yet";
  }

  const organizationCount = session.organizations.length;
  const courseCount = session.courses.length;
  return `${organizationCount} organization${organizationCount === 1 ? "" : "s"} and ${courseCount} course${courseCount === 1 ? "" : "s"} are visible`;
}
