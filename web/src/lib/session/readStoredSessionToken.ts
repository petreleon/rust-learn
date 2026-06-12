import { SESSION_SIGNED_OUT_KEY, SESSION_TOKEN_KEY } from "./SESSION_TOKEN_KEY";

export function readStoredSessionToken() {
  if (typeof window === "undefined") {
    return null;
  }

  const token = window.localStorage.getItem(SESSION_TOKEN_KEY);
  if (token) {
    return token;
  }

  if (window.localStorage.getItem(SESSION_SIGNED_OUT_KEY)) {
    window.sessionStorage.removeItem(SESSION_TOKEN_KEY);
    return null;
  }

  const legacyToken = window.sessionStorage.getItem(SESSION_TOKEN_KEY);
  if (legacyToken) {
    window.localStorage.setItem(SESSION_TOKEN_KEY, legacyToken);
    window.sessionStorage.removeItem(SESSION_TOKEN_KEY);
  }
  return legacyToken;
}
