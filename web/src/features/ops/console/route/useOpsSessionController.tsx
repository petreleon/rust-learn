"use client";

import { type FormEvent, useEffect, useState } from "react";
import { accessSummary } from "@/lib/access";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
  storeSessionToken,
  type CurrentSession,
} from "@/lib/session";
import { signInWithPassword } from "../api/opsConsoleTransport";
import { hasText } from "../model/hasText";
import { requestFailureMessage } from "../model/requestFailureMessage";
import { useOpsHealthController } from "./useOpsHealthController";
import { idleOpsResult, useOpsRequestController } from "./useOpsRequestController";

export function useOpsSessionController() {
  const [token, setToken] = useState("");
  const [showToken, setShowToken] = useState(false);
  const [credentials, setCredentials] = useState({ email: "", password: "" });
  const [sessionMessage, setSessionMessage] = useState("Not signed in");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [sessionLoading, setSessionLoading] = useState(true);
  const { apiMessage, apiRoot, apiState, checkHealth, updateApiRoot: setHealthApiRoot } = useOpsHealthController();
  const hasSessionToken = hasText(token);
  const request = useOpsRequestController({ apiRoot, hasSessionToken, token });
  const hasCompleteCredentials = hasText(credentials.email) && hasText(credentials.password);
  const canSignIn = hasCompleteCredentials && !request.hasPendingAction;
  const hasSessionDraft = hasSessionToken || hasText(credentials.email) || hasText(credentials.password);

  useEffect(() => {
    const storedToken = readStoredSessionToken();
    if (!storedToken) {
      setSessionLoading(false);
      return;
    }
    setToken(storedToken);
    verifySessionToken(storedToken, "Stored session expired or invalid").finally(() => setSessionLoading(false));
  }, []);

  async function verifySessionToken(nextToken: string, invalidMessage = "JWT loaded (Could not verify session)") {
    try {
      const nextSession = await fetchCurrentSession({ token: nextToken });
      setSession(nextSession);
      setSessionMessage(accessSummary(nextSession).platformAdmin ? "Signed in" : "Signed in (Not an operator)");
    } catch {
      setSession(null);
      setSessionMessage(invalidMessage);
      if (invalidMessage.includes("expired")) {
        clearStoredSessionToken();
        setToken("");
      }
    }
  }

  function updateToken(nextToken: string) {
    request.resetServerDenials();
    setToken(nextToken);
    if (!hasText(nextToken)) {
      clearStoredSessionToken();
      setSession(null);
      setShowToken(false);
      setSessionMessage("Session cleared");
      request.setResult({ ...idleOpsResult, body: "Session cleared. Previous API response hidden." });
      return;
    }
    storeSessionToken(nextToken);
    setSessionMessage("JWT loaded");
    void verifySessionToken(nextToken);
  }

  async function signIn(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!hasCompleteCredentials || !request.beginPendingAction("Sign in")) {
      setSessionMessage(
        hasCompleteCredentials ? `${request.pendingActionRef.current} request in progress` : "Email and password required",
      );
      return;
    }
    setSessionMessage("Signing in");
    request.setResult({ body: "Waiting for API response.", label: "Sign in", ok: true, status: "Pending" });
    try {
      const next = await signInWithPassword({ apiRoot, email: credentials.email, password: credentials.password });
      if (!next.ok) {
        setSessionMessage(next.body || `Sign in failed with ${next.status}`);
        request.setResult({ body: next.body, label: "Sign in", ok: false, status: next.status });
        return;
      }
      if (!next.token) {
        setSessionMessage("Sign in response did not include a JWT");
        request.setResult({
          body: "Sign in response did not include a JWT.",
          label: "Sign in",
          ok: false,
          status: next.status,
        });
        return;
      }
      setToken(next.token);
      storeSessionToken(next.token);
      setShowToken(false);
      request.resetServerDenials();
      setCredentials((current) => ({ ...current, password: "" }));
      request.setResult({ body: "JWT loaded into this session.", label: "Sign in", ok: true, status: next.status });
      void verifySessionToken(next.token);
    } catch (error) {
      const message = requestFailureMessage(error);
      setSessionMessage(message);
      request.setResult({ body: message, label: "Sign in", ok: false, status: "Request failed" });
    } finally {
      request.finishPendingAction("Sign in");
    }
  }

  function clearSession() {
    clearStoredSessionToken();
    setSession(null);
    setToken("");
    setShowToken(false);
    setCredentials({ email: "", password: "" });
    request.resetServerDenials();
    setSessionMessage("Session fields cleared");
    request.setResult({ ...idleOpsResult, body: "Session cleared. Previous API response hidden." });
  }

  function updateApiRoot(value: string) {
    request.resetServerDenials();
    setHealthApiRoot(value);
  }

  return {
    actionState: request.actionState,
    apiMessage,
    apiRoot,
    apiState,
    canSignIn,
    checkHealth,
    clearSession,
    credentials,
    hasCompleteCredentials,
    hasPendingAction: request.hasPendingAction,
    hasSessionDraft,
    hasSessionToken,
    isServerDenied: request.isServerDenied,
    pendingAction: request.pendingAction,
    pendingActionTitle: request.pendingActionTitle,
    result: request.result,
    session,
    sessionLoading,
    sessionMessage,
    setCredentials,
    setShowToken,
    showToken,
    signIn,
    token,
    updateApiRoot,
    updateToken,
    sendApi: request.sendApi,
  };
}
