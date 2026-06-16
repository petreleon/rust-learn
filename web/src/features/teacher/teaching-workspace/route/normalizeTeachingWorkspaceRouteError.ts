import { normalizeRouteError } from "@/shared/route-state/normalizeRouteError";

export function normalizeTeachingWorkspaceRouteError(error: unknown) {
  return normalizeRouteError(error, {
    code: "unexpected_error",
    message: "The teaching workspace request failed before RustLearn could finish loading.",
    status: 0,
  });
}
