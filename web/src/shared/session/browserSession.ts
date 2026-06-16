import { clearStoredSessionToken } from "@/lib/session/clearStoredSessionToken";
import { readStoredSessionToken } from "@/lib/session/readStoredSessionToken";

export function readBrowserSessionToken() {
  return readStoredSessionToken();
}

export function clearBrowserSession() {
  clearStoredSessionToken();
}
