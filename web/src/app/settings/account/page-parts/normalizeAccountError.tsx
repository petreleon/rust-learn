"use client";

import { LearnerRequestError } from "@/lib/learner";
import { SessionRequestError } from "@/lib/session";
import { type RouteError } from "./RouteError";

export function normalizeAccountError(error: unknown): RouteError {
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
