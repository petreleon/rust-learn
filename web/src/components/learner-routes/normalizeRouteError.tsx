"use client";

import { LearnerRequestError } from "@/lib/learner";
import { SessionRequestError } from "@/lib/session";

export function normalizeRouteError(error: unknown) {
  if (error instanceof SessionRequestError) {
    return error;
  }

  if (error instanceof LearnerRequestError) {
    return error;
  }

  return new LearnerRequestError("Learner route failed before the API responded.", 0, "network_error");
}
