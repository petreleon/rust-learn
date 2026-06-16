import { describe, expect, it } from "vitest";
import { isRequestError } from "../RequestError";

class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
    public code: string,
  ) {
    super(message);
  }
}

describe("isRequestError", () => {
  it("recognizes errors with status and code", () => {
    expect(isRequestError(new ApiError("Denied", 403, "permission_denied"))).toBe(true);
  });

  it("rejects unknown errors", () => {
    expect(isRequestError(new Error("boom"))).toBe(false);
  });
});
