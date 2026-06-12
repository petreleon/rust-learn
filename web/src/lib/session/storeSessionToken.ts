import { SESSION_SIGNED_OUT_KEY, SESSION_TOKEN_KEY } from "./SESSION_TOKEN_KEY";

export function storeSessionToken(token: string) {
  if (typeof window === "undefined") {
    return;
  }

  window.localStorage.setItem(SESSION_TOKEN_KEY, token);
  window.localStorage.removeItem(SESSION_SIGNED_OUT_KEY);
  window.sessionStorage.removeItem(SESSION_TOKEN_KEY);
}
