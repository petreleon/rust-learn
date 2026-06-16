"use client";

import { useCallback, useState } from "react";
import { type PlatformRewardCandidateItem } from "@/lib/admin/PlatformRewardCandidateItem";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { decideAdminRewardAmount } from "../api/rewardAmountReviewApi";
import {
  defaultRewardAmountDecisionDraft,
  isValidApprovedAmount,
  type RewardAmountDecisionDraft,
} from "../model/RewardAmountDecisionDraft";
import { type RewardAmountDecisionState } from "../model/RewardAmountDecisionState";
import { normalizeAdminRewardAmountRouteError } from "./normalizeAdminRewardAmountRouteError";

export function useRewardAmountDecision({
  onCandidateChanged,
  onConflict,
  onDecisionSaved,
  selectedCandidate,
}: {
  onCandidateChanged: () => Promise<void>;
  onConflict: (candidateId: number) => Promise<void>;
  onDecisionSaved: (candidateId: number) => Promise<void>;
  selectedCandidate: PlatformRewardCandidateItem | null;
}) {
  const [decisionDraft, setDecisionDraft] = useState<RewardAmountDecisionDraft>(
    defaultRewardAmountDecisionDraft,
  );
  const [decisionError, setDecisionError] = useState<RouteError | null>(null);
  const [decisionState, setDecisionState] = useState<RewardAmountDecisionState>("idle");

  const updateDecisionDraft = useCallback((patch: Partial<RewardAmountDecisionDraft>) => {
    setDecisionDraft((current) => ({ ...current, ...patch }));
  }, []);

  const handleDecision = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token || !selectedCandidate) return;

    const validationError = validateDecisionDraft(decisionDraft);
    if (validationError) {
      setDecisionError(validationError);
      setDecisionState("error");
      return;
    }

    setDecisionError(null);
    setDecisionState("submitting");

    try {
      await decideAdminRewardAmount({
        approvedAmount: decisionDraft.status === "approved" ? decisionDraft.amount.trim() : null,
        candidateId: selectedCandidate.id,
        decisionReason: decisionDraft.reason.trim(),
        status: decisionDraft.status,
        token,
      });
      setDecisionDraft(defaultRewardAmountDecisionDraft);
      setDecisionState("success");
      await onCandidateChanged();
      await onDecisionSaved(selectedCandidate.id);
    } catch (nextError) {
      const routeError = normalizeAdminRewardAmountRouteError(
        nextError,
        "Reward amount decision could not be saved.",
      );
      setDecisionError(routeError);
      setDecisionState("error");
      if (routeError.status === 409) {
        await onCandidateChanged();
        await onConflict(selectedCandidate.id);
      }
    }
  }, [decisionDraft, onCandidateChanged, onConflict, onDecisionSaved, selectedCandidate]);

  return {
    decisionDraft,
    decisionError,
    decisionState,
    handleDecision,
    updateDecisionDraft,
  };
}

function validateDecisionDraft(draft: RewardAmountDecisionDraft): RouteError | null {
  if (!draft.reason.trim()) {
    return {
      code: "validation_error",
      message: "Add a decision reason before changing a candidate.",
      status: 400,
    };
  }
  if (draft.status === "approved" && !isValidApprovedAmount(draft.amount)) {
    return {
      code: "validation_error",
      message: "Enter a valid non-negative approved amount.",
      status: 400,
    };
  }
  return null;
}
