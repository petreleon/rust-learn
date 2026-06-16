"use client";

import { useCallback, useState } from "react";
import { type TeacherApplicationAuditEvent } from "@/lib/admin/TeacherApplicationAuditEvent";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminTeacherApplicationAudit } from "../api/teacherApplicationsApi";
import { normalizeAdminTeacherApplicationRouteError } from "./normalizeAdminTeacherApplicationRouteError";

export function useTeacherApplicationAudit({ canReview }: { canReview: boolean }) {
  const [auditError, setAuditError] = useState<RouteError | null>(null);
  const [auditEvents, setAuditEvents] = useState<TeacherApplicationAuditEvent[]>([]);
  const [auditState, setAuditState] = useState<LoadState>("idle");

  const resetAudit = useCallback(() => {
    setAuditError(null);
    setAuditEvents([]);
    setAuditState("idle");
  }, []);

  const loadAudit = useCallback(
    async (applicationId: number) => {
      const token = readBrowserSessionToken();
      if (!token || !canReview) return;

      setAuditState("loading");
      setAuditError(null);

      try {
        setAuditEvents(await loadAdminTeacherApplicationAudit({ applicationId, token }));
        setAuditState("success");
      } catch (nextError) {
        setAuditEvents([]);
        setAuditError(
          normalizeAdminTeacherApplicationRouteError(
            nextError,
            "Teacher application audit could not be loaded.",
          ),
        );
        setAuditState("error");
      }
    },
    [canReview],
  );

  return {
    auditError,
    auditEvents,
    auditState,
    loadAudit,
    resetAudit,
  };
}
