"use client";

import { type CurrentSession } from "@/lib/session";

export function buildWorkspaceOptions(session?: CurrentSession | null) {
  if (!session) {
    return [];
  }

  const options = [{ label: "Personal workspace", value: `user:${session.user.id}` }];
  options.push(
    ...session.organizations.map((organization) => ({
      label: organization.name,
      value: `organization:${organization.id}`,
    })),
  );
  options.push(
    ...session.courses.map((course) => ({
      label: course.title,
      value: `course:${course.id}`,
    })),
  );
  return options;
}
