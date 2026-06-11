"use client";

import { type CourseCatalogItem } from "@/lib/learner";

export function courseOrganizationLabel(course: CourseCatalogItem) {
  return course.organizations.map((organization) => organization.name).join(", ") || "Independent";
}
