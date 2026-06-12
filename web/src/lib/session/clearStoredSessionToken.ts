import { SESSION_SIGNED_OUT_KEY, SESSION_TOKEN_KEY } from "./SESSION_TOKEN_KEY";

export function clearStoredSessionToken() {
  if (typeof window === "undefined") {
    return;
  }

  window.localStorage.removeItem(SESSION_TOKEN_KEY);
  window.localStorage.setItem(SESSION_SIGNED_OUT_KEY, new Date().toISOString());
  window.sessionStorage.removeItem(SESSION_TOKEN_KEY);
}
