"use client";

import { normalizeRouteError as normalizeSharedRouteError } from "@/shared/route-state/normalizeRouteError";
import { type RouteError } from "./RouteError";

export function normalizeRouteError(error: unknown): RouteError {
  return normalizeSharedRouteError(error, {
    code: "unexpected_error",
    message: "The teaching workspace request failed before RustLearn could finish loading.",
    status: 0,
  });
}
