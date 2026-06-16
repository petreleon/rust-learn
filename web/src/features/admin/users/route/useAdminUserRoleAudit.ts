"use client";

import { useCallback, useState } from "react";
import { type AdminUserRoleAssignmentAuditEvent } from "@/lib/admin";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminUserRoleAssignmentAudit } from "../api/adminUsersApi";
import { normalizeAdminUsersRouteError } from "./normalizeAdminUsersRouteError";

export function useAdminUserRoleAudit(canViewAudit: boolean) {
  const [events, setEvents] = useState<AdminUserRoleAssignmentAuditEvent[]>([]);
  const [error, setError] = useState<RouteError | null>(null);
  const [state, setState] = useState<LoadState>("idle");

  const resetAudit = useCallback(() => {
    setEvents([]);
    setError(null);
    setState("idle");
  }, []);

  const loadAudit = useCallback(
    async (userId: number, providedToken?: string | null) => {
      const token = providedToken ?? readBrowserSessionToken();
      if (!token || !canViewAudit) {
        resetAudit();
        return;
      }
      setError(null);
      setState("loading");
      try {
        setEvents(await loadAdminUserRoleAssignmentAudit({ token, userId }));
        setState("success");
      } catch (nextError) {
        setError(normalizeAdminUsersRouteError(nextError, "Role assignment history could not be loaded."));
        setState("error");
      }
    },
    [canViewAudit, resetAudit],
  );

  return { error, events, loadAudit, resetAudit, state };
}
