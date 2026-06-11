"use client";

import { newIdempotencyKey } from "./newIdempotencyKey";
import { type ApplicationDraft } from "./ApplicationDraft";

export function defaultDraft(): ApplicationDraft {
  return {
    experienceSummary: "",
    idempotencyKey: newIdempotencyKey(),
    portfolioLinks: "",
    requestedCourseId: "",
    requestedOrganizationId: "",
    requestedScope: "platform",
  };
}
