import { beforeAll, beforeEach, describe, expect, it } from "vitest";
import {
  SESSION_TOKEN_KEY,
  SESSION_SIGNED_OUT_KEY,
  readStoredSessionToken,
  storeSessionToken,
  clearStoredSessionToken,
  SessionRequestError,
} from "@/lib/session";

function memoryStorage(): Storage {
  const values = new Map<string, string>();
  return {
    clear: () => values.clear(),
    getItem: (key) => values.get(key) ?? null,
    key: (index) => Array.from(values.keys())[index] ?? null,
    get length() {
      return values.size;
    },
    removeItem: (key) => values.delete(key),
    setItem: (key, value) => values.set(key, value),
  };
}

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
  beforeAll(() => {
    Object.defineProperty(window, "localStorage", { configurable: true, value: memoryStorage() });
    Object.defineProperty(window, "sessionStorage", { configurable: true, value: memoryStorage() });
  });

  beforeEach(() => {
    window.localStorage.clear();
    window.sessionStorage.clear();
  });

  it("readStoredSessionToken returns null when empty", () => {
    expect(readStoredSessionToken()).toBeNull();
  });

  it("storeSessionToken and readStoredSessionToken roundtrip", () => {
    storeSessionToken("my-jwt-token");
    expect(readStoredSessionToken()).toBe("my-jwt-token");
    expect(window.localStorage.getItem(SESSION_TOKEN_KEY)).toBe("my-jwt-token");
    expect(window.localStorage.getItem(SESSION_SIGNED_OUT_KEY)).toBeNull();
  });

  it("storeSessionToken overwrites previous", () => {
    storeSessionToken("first");
    storeSessionToken("second");
    expect(readStoredSessionToken()).toBe("second");
  });

  it("clearStoredSessionToken removes token", () => {
    storeSessionToken("token");
    window.sessionStorage.setItem(SESSION_TOKEN_KEY, "legacy-token");
    clearStoredSessionToken();
    expect(readStoredSessionToken()).toBeNull();
    expect(window.sessionStorage.getItem(SESSION_TOKEN_KEY)).toBeNull();
    expect(window.localStorage.getItem(SESSION_SIGNED_OUT_KEY)).toBeTruthy();
  });

  it("migrates legacy sessionStorage tokens into shared localStorage", () => {
    window.sessionStorage.setItem(SESSION_TOKEN_KEY, "legacy-token");

    expect(readStoredSessionToken()).toBe("legacy-token");
    expect(window.localStorage.getItem(SESSION_TOKEN_KEY)).toBe("legacy-token");
    expect(window.sessionStorage.getItem(SESSION_TOKEN_KEY)).toBeNull();
  });

  it("does not migrate legacy tokens after sign out", () => {
    window.sessionStorage.setItem(SESSION_TOKEN_KEY, "legacy-token");
    clearStoredSessionToken();
    window.sessionStorage.setItem(SESSION_TOKEN_KEY, "stale-legacy-token");

    expect(readStoredSessionToken()).toBeNull();
    expect(window.localStorage.getItem(SESSION_TOKEN_KEY)).toBeNull();
    expect(window.sessionStorage.getItem(SESSION_TOKEN_KEY)).toBeNull();
  });

  it("clearStoredSessionToken is idempotent", () => {
    clearStoredSessionToken();
    expect(readStoredSessionToken()).toBeNull();
  });
});

describe("SESSION_TOKEN_KEY", () => {
  it("has a consistent key name", () => {
    expect(SESSION_TOKEN_KEY).toBe("rustlearn.session.jwt");
    expect(SESSION_SIGNED_OUT_KEY).toBe("rustlearn.session.signedOutAt");
  });
});
