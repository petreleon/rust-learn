"use client";

import { DRAFT_STORAGE_KEY } from "./DRAFT_STORAGE_KEY";
import { defaultDraft } from "./defaultDraft";
import { isTeacherScope } from "./isTeacherScope";
import { newIdempotencyKey } from "./newIdempotencyKey";
import { type ApplicationDraft } from "./ApplicationDraft";

export function readDraft(): ApplicationDraft {
  if (typeof window === "undefined") {
    return defaultDraft();
  }

  const raw = window.sessionStorage.getItem(DRAFT_STORAGE_KEY);
  if (!raw) {
    return defaultDraft();
  }

  try {
    const parsed = JSON.parse(raw) as Partial<ApplicationDraft>;
    return {
      experienceSummary: parsed.experienceSummary || "",
      idempotencyKey: parsed.idempotencyKey || newIdempotencyKey(),
      portfolioLinks: parsed.portfolioLinks || "",
      requestedCourseId: parsed.requestedCourseId || "",
      requestedOrganizationId: parsed.requestedOrganizationId || "",
      requestedScope: isTeacherScope(parsed.requestedScope) ? parsed.requestedScope : "platform",
    };
  } catch {
    return defaultDraft();
  }
}
