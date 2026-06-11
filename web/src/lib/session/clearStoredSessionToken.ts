import { SESSION_TOKEN_KEY } from "./SESSION_TOKEN_KEY";

export function clearStoredSessionToken() {
  if (typeof window === "undefined") {
    return;
  }

  window.sessionStorage.removeItem(SESSION_TOKEN_KEY);
}
