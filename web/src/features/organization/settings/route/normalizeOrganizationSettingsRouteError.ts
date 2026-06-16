import { normalizeRouteError } from "@/shared/route-state/normalizeRouteError";
import { type RouteError } from "@/shared/route-state/RouteError";

export function normalizeOrganizationSettingsRouteError(error: unknown): RouteError {
  return normalizeRouteError(error, {
    code: "network_error",
    message: "Organization settings could not be loaded.",
    status: 0,
  });
}
