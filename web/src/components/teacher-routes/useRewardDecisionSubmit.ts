"use client";

import { type FormEvent, useState } from "react";
import { readStoredSessionToken } from "@/lib/session";
import { decideTeacherRewardCandidate, type TeacherRewardCandidate } from "@/lib/teacher";
import { defaultRewardDecisionDraft } from "./defaultRewardDecisionDraft";
import { normalizeRouteError } from "./normalizeRouteError";
import { type ActionState } from "./ActionState";
import { type RewardDecisionDraft } from "./RewardDecisionDraft";

export function useRewardDecisionSubmit({
  courseId,
  onReload,
}: {
  courseId: string;
  onReload: () => Promise<void>;
}) {
  const [actionMessage, setActionMessage] = useState<string | null>(null);
  const [actionState, setActionState] = useState<ActionState>("idle");
  const [decisionDrafts, setDecisionDrafts] = useState<Record<number, RewardDecisionDraft>>({});

  function updateDecisionDraft(candidateId: number, draft: RewardDecisionDraft) {
    setDecisionDrafts((current) => ({ ...current, [candidateId]: draft }));
  }

  async function submitRewardDecision(candidate: TeacherRewardCandidate, event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readStoredSessionToken();
    const draft = decisionDrafts[candidate.id] || defaultRewardDecisionDraft;
    if (!token) {
      setActionMessage("Sign in again before deciding reward candidates.");
      return;
    }
    setActionState("saving");
    setActionMessage(null);
    try {
      await decideTeacherRewardCandidate({
        candidateId: candidate.id,
        courseId,
        payload: { decision_reason: draft.reason.trim() || null, status: draft.status },
        token,
      });
      setDecisionDrafts((current) => {
        const next = { ...current };
        delete next[candidate.id];
        return next;
      });
      setActionMessage("Reward candidate decision saved.");
      await onReload();
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      setActionMessage(routeError.status === 409 ? `${routeError.message} The queue has been refreshed.` : routeError.message);
      if (routeError.status === 409) await onReload();
    } finally {
      setActionState("idle");
    }
  }

  return { actionMessage, actionState, decisionDrafts, submitRewardDecision, updateDecisionDraft };
}
