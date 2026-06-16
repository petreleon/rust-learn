import { AdminRequestError } from "@/lib/admin";
import { type RouteError } from "@/shared/route-state/RouteError";

export function normalizeAdminUsersRouteError(error: unknown, fallback: string): RouteError {
  if (error instanceof AdminRequestError) {
    return { code: error.code, message: error.message, status: error.status };
  }

  return { code: "unknown_error", message: fallback, status: 0 };
}
