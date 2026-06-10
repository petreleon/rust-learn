import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  SESSION_TOKEN_KEY,
  readStoredSessionToken,
  storeSessionToken,
  clearStoredSessionToken,
  SessionRequestError,
} from "@/lib/session";

describe("SessionRequestError", () => {
  it("constructs with status and code", () => {
    const err = new SessionRequestError("msg", 401, "unauthorized");
    expect(err.message).toBe("msg");
    expect(err.status).toBe(401);
    expect(err.code).toBe("unauthorized");
    expect(err.name).toBe("SessionRequestError");
    expect(err).toBeInstanceOf(Error);
  });
});

describe("Session token storage", () => {
  beforeEach(() => {
    sessionStorage.clear();
  });

  it("readStoredSessionToken returns null when empty", () => {
    expect(readStoredSessionToken()).toBeNull();
  });

  it("storeSessionToken and readStoredSessionToken roundtrip", () => {
    storeSessionToken("my-jwt-token");
    expect(readStoredSessionToken()).toBe("my-jwt-token");
  });

  it("storeSessionToken overwrites previous", () => {
    storeSessionToken("first");
    storeSessionToken("second");
    expect(readStoredSessionToken()).toBe("second");
  });

  it("clearStoredSessionToken removes token", () => {
    storeSessionToken("token");
    clearStoredSessionToken();
    expect(readStoredSessionToken()).toBeNull();
  });

  it("clearStoredSessionToken is idempotent", () => {
    clearStoredSessionToken();
    expect(readStoredSessionToken()).toBeNull();
  });
});

describe("SESSION_TOKEN_KEY", () => {
  it("has a consistent key name", () => {
    expect(SESSION_TOKEN_KEY).toBe("rustlearn.session.jwt");
  });
});
