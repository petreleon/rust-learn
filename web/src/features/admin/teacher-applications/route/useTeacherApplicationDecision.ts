"use client";

import { useCallback, useState } from "react";
import { type PlatformTeacherApplicationItem } from "@/lib/admin/PlatformTeacherApplicationItem";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { decideAdminTeacherApplication } from "../api/teacherApplicationsApi";
import {
  defaultTeacherApplicationDecisionDraft,
  type TeacherApplicationDecisionDraft,
} from "../model/TeacherApplicationDecisionDraft";
import { type TeacherApplicationDecisionState } from "../model/TeacherApplicationDecisionState";
import { normalizeAdminTeacherApplicationRouteError } from "./normalizeAdminTeacherApplicationRouteError";

export function useTeacherApplicationDecision({
  onApplicationChanged,
  onConflict,
  onDecisionSaved,
  selectedApplication,
}: {
  onApplicationChanged: () => Promise<void>;
  onConflict: (applicationId: number) => Promise<void>;
  onDecisionSaved: (applicationId: number) => Promise<void>;
  selectedApplication: PlatformTeacherApplicationItem | null;
}) {
  const [decisionDraft, setDecisionDraft] = useState<TeacherApplicationDecisionDraft>(
    defaultTeacherApplicationDecisionDraft,
  );
  const [decisionError, setDecisionError] = useState<RouteError | null>(null);
  const [decisionState, setDecisionState] = useState<TeacherApplicationDecisionState>("idle");

  const updateDecisionDraft = useCallback((patch: Partial<TeacherApplicationDecisionDraft>) => {
    setDecisionDraft((current) => ({ ...current, ...patch }));
  }, []);

  const handleDecision = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token || !selectedApplication) return;

    const validationError = validateDecisionDraft(decisionDraft);
    if (validationError) {
      setDecisionError(validationError);
      setDecisionState("error");
      return;
    }

    setDecisionError(null);
    setDecisionState("submitting");

    try {
      await decideAdminTeacherApplication({
        applicationId: selectedApplication.id,
        decisionReason: decisionDraft.reason.trim(),
        status: decisionDraft.status,
        token,
      });
      setDecisionDraft(defaultTeacherApplicationDecisionDraft);
      setDecisionState("success");
      await onApplicationChanged();
      await onDecisionSaved(selectedApplication.id);
    } catch (nextError) {
      const routeError = normalizeAdminTeacherApplicationRouteError(
        nextError,
        "Teacher application decision could not be saved.",
      );
      setDecisionError(routeError);
      setDecisionState("error");
      if (routeError.status === 409) {
        await onApplicationChanged();
        await onConflict(selectedApplication.id);
      }
    }
  }, [decisionDraft, onApplicationChanged, onConflict, onDecisionSaved, selectedApplication]);

  return {
    decisionDraft,
    decisionError,
    decisionState,
    handleDecision,
    updateDecisionDraft,
  };
}

function validateDecisionDraft(draft: TeacherApplicationDecisionDraft): RouteError | null {
  if (!draft.reason.trim()) {
    return {
      code: "validation_error",
      message: "Add a decision reason before changing an application.",
      status: 400,
    };
  }
  return null;
}
