"use client";

import { API_REQUEST_TIMEOUT_SECONDS } from "./API_REQUEST_TIMEOUT_SECONDS";

export function requestFailureMessage(error: unknown) {
  if (error instanceof Error && error.name === "AbortError") {
    return `Request timed out after ${API_REQUEST_TIMEOUT_SECONDS} seconds.`;
  }

  return error instanceof Error ? error.message : "Unknown request failure";
}
