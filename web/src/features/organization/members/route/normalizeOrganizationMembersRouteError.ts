import { normalizeRouteError } from "@/shared/route-state/normalizeRouteError";
import { type RouteError } from "@/shared/route-state/RouteError";

export function normalizeOrganizationMembersRouteError(error: unknown): RouteError {
  return normalizeRouteError(error, {
    code: "network_error",
    message: "Organization members could not be loaded.",
    status: 0,
  });
}
