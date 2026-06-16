"use client";

import { AdminRequestError } from "@/lib/admin";
import { SessionRequestError } from "@/lib/session";
import { type RouteError } from "./RouteError";

export function normalizeRouteError(error: unknown, fallbackMessage: string): RouteError {
  if (error instanceof SessionRequestError || error instanceof AdminRequestError) {
    return {
      code: error.code,
      message: error.message,
      status: error.status,
    };
  }

  return {
    code: "network_error",
    message: fallbackMessage,
    status: 0,
  };
}
