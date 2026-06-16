"use client";

import { SessionRequestError } from "@/lib/session";
import { TeacherRequestError } from "@/lib/teacher";
import { type RouteError } from "../model/RouteError";

export function normalizeApplicationRouteError(error: unknown): RouteError {
  if (error instanceof TeacherRequestError || error instanceof SessionRequestError) {
    return {
      code: error.code,
      message: error.message,
      status: error.status,
    };
  }

  return {
    code: "unexpected_error",
    message: "The request failed before RustLearn could finish loading the application state.",
    status: 0,
  };
}
