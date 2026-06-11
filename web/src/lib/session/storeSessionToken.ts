import { SESSION_TOKEN_KEY } from "./SESSION_TOKEN_KEY";

export function storeSessionToken(token: string) {
  if (typeof window === "undefined") {
    return;
  }

  window.sessionStorage.setItem(SESSION_TOKEN_KEY, token);
}
