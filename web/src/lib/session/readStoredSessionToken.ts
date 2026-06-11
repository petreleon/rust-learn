import { SESSION_TOKEN_KEY } from "./SESSION_TOKEN_KEY";

export function readStoredSessionToken() {
  if (typeof window === "undefined") {
    return null;
  }

  return window.sessionStorage.getItem(SESSION_TOKEN_KEY);
}
