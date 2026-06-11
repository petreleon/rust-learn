"use client";

import { type CurrentSession } from "@/lib/session";
import { parsePortfolioLinks } from "./parsePortfolioLinks";
import { type ApplicationDraft } from "./ApplicationDraft";
import { type ValidationResult } from "./ValidationResult";

export function validateDraft(draft: ApplicationDraft, session: CurrentSession | null): ValidationResult {
  const errors: string[] = [];
  const portfolioLinks = parsePortfolioLinks(draft.portfolioLinks);

  if (!draft.experienceSummary.trim()) {
    errors.push("Add an experience summary.");
  }

  if (draft.requestedScope === "organization") {
    const hasOrganization = session?.organizations.some(
      (organization) => String(organization.id) === draft.requestedOrganizationId,
    );
    if (!hasOrganization) {
      errors.push("Choose an organization from your session context.");
    }
  }

  if (draft.requestedScope === "course") {
    const hasCourse = session?.courses.some((course) => String(course.id) === draft.requestedCourseId);
    if (!hasCourse) {
      errors.push("Choose a course from your session context.");
    }
  }

  return { errors, portfolioLinks };
}
