"use client";

import { OrganizationRequestError } from "@/lib/organization";
import { SessionRequestError } from "@/lib/session";
import { type RouteError } from "./RouteError";

export function normalizeRouteError(error: unknown): RouteError {
  if (error instanceof SessionRequestError) {
    return {
      code: error.code,
      message: error.message,
      status: error.status,
    };
  }

  if (error instanceof OrganizationRequestError) {
    if (error.status === 404 && error.code === "course_not_found") {
      return {
        code: "course_not_found",
        message: "This course no longer exists. It may have been deleted; reload the course list before continuing.",
        status: 404,
      };
    }

    return {
      code: error.code,
      message: error.message,
      status: error.status,
    };
  }

  return {
    code: "network_error",
    message: "Organization workspace could not be loaded.",
    status: 0,
  };
}
