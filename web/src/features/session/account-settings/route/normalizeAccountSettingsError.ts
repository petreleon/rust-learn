"use client";

import { LearnerRequestError } from "@/lib/learner/LearnerRequestError";
import { SessionRequestError } from "@/lib/session/SessionRequestError";
import { type RouteError } from "../model/RouteError";

export function normalizeAccountSettingsError(error: unknown): RouteError {
  if (error instanceof SessionRequestError || error instanceof LearnerRequestError) {
    return {
      code: error.code,
      message: error.message,
      status: error.status,
    };
  }

  return {
    code: "network_error",
    message: "Account settings could not be loaded before the API responded.",
    status: 0,
  };
}
