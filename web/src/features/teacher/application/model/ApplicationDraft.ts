"use client";

import { type TeacherApplicationScope } from "@/lib/teacher";

export type ApplicationDraft = {
  experienceSummary: string;
  idempotencyKey: string;
  portfolioLinks: string;
  requestedCourseId: string;
  requestedOrganizationId: string;
  requestedScope: TeacherApplicationScope;
};
