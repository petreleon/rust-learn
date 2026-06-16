import { SessionRequestError } from "@/lib/session/SessionRequestError";
import { type RouteError } from "../model/RouteError";

export function normalizeSessionWorkspaceRouteError(error: unknown): RouteError {
  if (error instanceof SessionRequestError) {
    return {
      code: error.code,
      message: error.message,
      status: error.status,
    };
  }

  return {
    code: "network_error",
    message: "Session request failed.",
    status: 0,
  };
}
