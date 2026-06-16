"use client";

import { useRef, useState } from "react";
import { sendOpsApiRequest } from "../api/opsConsoleTransport";
import { type ApiResult } from "../model/ApiResult";
import { type HttpMethod } from "../model/HttpMethod";
import { requestFailureMessage } from "../model/requestFailureMessage";

export const idleOpsResult: ApiResult = {
  body: "No request sent.",
  label: "Result",
  ok: true,
  status: "Idle",
};

export function useOpsRequestController({
  apiRoot,
  hasSessionToken,
  token,
}: {
  apiRoot: string;
  hasSessionToken: boolean;
  token: string;
}) {
  const pendingActionRef = useRef<string | null>(null);
  const [serverDeniedActions, setServerDeniedActions] = useState<Set<string>>(() => new Set());
  const [pendingAction, setPendingAction] = useState<string | null>(null);
  const [result, setResult] = useState<ApiResult>(idleOpsResult);
  const hasPendingAction = pendingAction !== null;
  const pendingActionTitle = pendingAction ? `${pendingAction} request in progress` : undefined;

  function resetServerDenials() {
    setServerDeniedActions(new Set());
  }

  function setServerDeniedAction(action: string, denied: boolean) {
    setServerDeniedActions((current) => {
      if (current.has(action) === denied) {
        return current;
      }
      const next = new Set(current);
      if (denied) {
        next.add(action);
      } else {
        next.delete(action);
      }
      return next;
    });
  }

  function beginPendingAction(action: string) {
    if (pendingActionRef.current) {
      return false;
    }
    pendingActionRef.current = action;
    setPendingAction(action);
    return true;
  }

  function finishPendingAction(action: string) {
    if (pendingActionRef.current !== action) {
      return;
    }
    pendingActionRef.current = null;
    setPendingAction(null);
  }

  async function sendApi(label: string, path: string, method: HttpMethod = "GET", body?: unknown) {
    const revealResultPanel = () => {
      requestAnimationFrame(() => document.getElementById("result-panel")?.scrollIntoView({ block: "start", inline: "nearest" }));
    };
    if (!hasSessionToken || !beginPendingAction(label)) {
      setResult({
        body: hasSessionToken
          ? `${pendingActionRef.current} is already waiting for an API response.`
          : "Add a JWT before sending protected API requests.",
        label,
        ok: false,
        status: hasSessionToken ? "Request pending" : "Session required",
      });
      revealResultPanel();
      return;
    }

    setResult({ body: "Waiting for API response.", label, ok: true, status: "Pending" });
    revealResultPanel();
    try {
      const response = await sendOpsApiRequest({ apiRoot, body, method, path, token });
      setServerDeniedAction(label, response.status === 403);
      setResult({
        body:
          response.status === 403
            ? [response.body, "This JWT does not grant server access for this action."].filter(Boolean).join("\n\n")
            : response.body,
        label,
        ok: response.ok,
        status: `HTTP ${response.status}`,
      });
      revealResultPanel();
    } catch (error) {
      setResult({ body: requestFailureMessage(error), label, ok: false, status: "Request failed" });
      revealResultPanel();
    } finally {
      finishPendingAction(label);
    }
  }

  function actionState(ready = true, allowed = true, permissionLabel = "Required permission", serverAction?: string) {
    if (serverAction && serverDeniedActions.has(serverAction)) {
      return { disabled: true, title: "Server denied this JWT" };
    }
    if (!allowed || !hasSessionToken || !ready || hasPendingAction) {
      const title = !allowed
        ? permissionLabel
        : !hasSessionToken
          ? "JWT required"
          : !ready
            ? "Complete required fields"
            : pendingActionTitle;
      return { disabled: true, title };
    }
    return { disabled: false, title: undefined };
  }

  return {
    actionState,
    beginPendingAction,
    finishPendingAction,
    hasPendingAction,
    isServerDenied: (action: string) => serverDeniedActions.has(action),
    pendingAction,
    pendingActionRef,
    pendingActionTitle,
    resetServerDenials,
    result,
    sendApi,
    setResult,
  };
}
