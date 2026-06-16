import { describe, expect, it } from "vitest";
import { normalizeRouteError } from "../normalizeRouteError";

class RequestError extends Error {
  constructor(
    message: string,
    public status: number,
    public code: string,
  ) {
    super(message);
  }
}

describe("normalizeRouteError", () => {
  it("preserves request error status, code, and message", () => {
    expect(normalizeRouteError(new RequestError("Denied", 403, "permission_denied"), fallback())).toEqual({
      code: "permission_denied",
      message: "Denied",
      status: 403,
    });
  });

  it("uses the fallback for unknown errors", () => {
    expect(normalizeRouteError(new Error("boom"), fallback())).toEqual(fallback());
  });
});

function fallback() {
  return {
    code: "unexpected_error",
    message: "Route failed.",
    status: 0,
  };
}
