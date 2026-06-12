"use client";

import { useCallback, useEffect, useState } from "react";
import { fetchKycSubmissionAudit, type KycAuditEvent } from "@/lib/admin";
import { normalizeRouteError } from "./normalizeRouteError";
import { type RouteError } from "./RouteError";
import { type SectionState } from "./SectionState";

export function useKycReviewAudit({
  canReview,
  submissionId,
  token,
}: {
  canReview: boolean;
  submissionId: number | null;
  token?: string;
}) {
  const [auditEvents, setAuditEvents] = useState<KycAuditEvent[]>([]);
  const [auditError, setAuditError] = useState<RouteError | null>(null);
  const [auditState, setAuditState] = useState<SectionState>("idle");

  const loadAudit = useCallback(async () => {
    if (!token || !canReview || !submissionId) {
      return;
    }
    setAuditState("loading");
    setAuditError(null);
    try {
      const events = await fetchKycSubmissionAudit({ submissionId, token });
      setAuditEvents(events);
      setAuditState("success");
    } catch (error) {
      setAuditEvents([]);
      setAuditError(normalizeRouteError(error, "KYC audit history could not be loaded."));
      setAuditState("error");
    }
  }, [canReview, submissionId, token]);

  useEffect(() => {
    if (!submissionId) {
      const timeout = window.setTimeout(() => {
        setAuditEvents([]);
        setAuditError(null);
        setAuditState("idle");
      }, 0);
      return () => window.clearTimeout(timeout);
    }
    const timeout = window.setTimeout(() => void loadAudit(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadAudit, submissionId]);

  return { auditError, auditEvents, auditState, loadAudit };
}
