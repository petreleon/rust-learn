import { normalizeRouteError } from "@/shared/route-state/normalizeRouteError";
import { type RouteError } from "@/shared/route-state/RouteError";

export function normalizeCourseWorkspaceRouteError(error: unknown): RouteError {
  const routeError = normalizeRouteError(error, {
    code: "unexpected_error",
    message: "The teaching workspace request failed before RustLearn could finish loading.",
    status: 0,
  });
  if (routeError.status === 404 && routeError.code === "course_not_found") {
    return {
      code: "course_not_found",
      message: "This course no longer exists. It may have been deleted; return to your teaching courses before continuing.",
      status: 404,
    };
  }
  return routeError;
}
