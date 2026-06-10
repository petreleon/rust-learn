import { describe, it, expect } from "vitest";
import { AuthRequestError } from "@/lib/auth";

describe("AuthRequestError", () => {
  it("constructs with message, status, and code", () => {
    const err = new AuthRequestError("Invalid credentials", 401, "invalid_credentials");
    expect(err.message).toBe("Invalid credentials");
    expect(err.status).toBe(401);
    expect(err.code).toBe("invalid_credentials");
    expect(err.name).toBe("AuthRequestError");
    expect(err).toBeInstanceOf(Error);
  });

  it("supports network error codes", () => {
    const err = new AuthRequestError("timeout", 0, "timeout");
    expect(err.status).toBe(0);
    expect(err.code).toBe("timeout");
  });
});
