import { type RouteError } from "./RouteError";
import { isRequestError } from "../api/RequestError";

export function normalizeRouteError(error: unknown, fallback: RouteError): RouteError {
  if (isRequestError(error)) {
    return {
      code: error.code,
      message: error.message,
      status: error.status,
    };
  }

  return fallback;
}
