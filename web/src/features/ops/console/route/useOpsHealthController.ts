"use client";

import { useEffect, useState } from "react";
import { HEALTH_CHECK_TIMEOUT_MS } from "../model/HEALTH_CHECK_TIMEOUT_MS";
import { type ApiState } from "../model/ApiState";
import { formatHealthMessage } from "../model/formatHealthMessage";
import { normalizeRoot } from "../model/normalizeRoot";

export function useOpsHealthController() {
  const [apiRoot, setApiRoot] = useState(process.env.NEXT_PUBLIC_API_URL || "/api");
  const [healthCheckTick, setHealthCheckTick] = useState(0);
  const [apiState, setApiState] = useState<ApiState>("checking");
  const [apiMessage, setApiMessage] = useState("Checking API");

  useEffect(() => {
    const controller = new AbortController();
    const root = normalizeRoot(apiRoot);
    const healthRoot = root.endsWith("/api") ? root.slice(0, -4) : root;
    let timedOut = false;
    const timeoutId = window.setTimeout(() => {
      timedOut = true;
      controller.abort();
    }, HEALTH_CHECK_TIMEOUT_MS);

    fetch(`${healthRoot || ""}/health`, { signal: controller.signal })
      .then(async (response) => {
        const body = await response.text();
        window.clearTimeout(timeoutId);
        if (!response.ok) {
          throw new Error(`HTTP ${response.status}`);
        }
        setApiState("online");
        setApiMessage(formatHealthMessage(body));
      })
      .catch((error: Error) => {
        window.clearTimeout(timeoutId);
        if (controller.signal.aborted && !timedOut) {
          return;
        }
        setApiState("offline");
        setApiMessage(timedOut ? "API health check timed out" : error.message || "Connection failed");
      });

    return () => {
      window.clearTimeout(timeoutId);
      controller.abort();
    };
  }, [apiRoot, healthCheckTick]);

  function updateApiRoot(value: string) {
    setApiState("checking");
    setApiMessage("Checking API");
    setApiRoot(value);
  }

  function checkHealth() {
    setApiState("checking");
    setApiMessage("Checking API");
    setHealthCheckTick((current) => current + 1);
  }

  return { apiMessage, apiRoot, apiState, checkHealth, updateApiRoot };
}
