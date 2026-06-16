import { normalizeRouteError } from "@/shared/route-state/normalizeRouteError";
import { type RouteError } from "@/shared/route-state/RouteError";

export function normalizeRewardRouteError(error: unknown): RouteError {
  return normalizeRouteError(error, {
    code: "unexpected_error",
    message: "The teaching reward review request failed before RustLearn could finish loading.",
    status: 0,
  });
}
