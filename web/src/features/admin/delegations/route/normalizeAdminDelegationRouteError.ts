import { AdminRequestError } from "@/lib/admin/AdminRequestError";
import { SessionRequestError } from "@/lib/session/SessionRequestError";
import { type RouteError } from "@/shared/route-state/RouteError";

export function normalizeAdminDelegationRouteError(error: unknown, fallbackMessage: string): RouteError {
  if (error instanceof AdminRequestError || error instanceof SessionRequestError) {
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
