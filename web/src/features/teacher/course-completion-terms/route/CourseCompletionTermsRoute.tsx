"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";
import { type ActionState } from "@/shared/route-state/ActionState";
import { normalizeRouteError } from "@/shared/route-state/normalizeRouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  acceptCourseCompletionTerms,
  counterCourseCompletionTerms,
  loadCourseCompletionTerms,
  rejectCourseCompletionTerms,
  submitCourseCompletionTerms,
  withdrawCourseCompletionTerms,
} from "../api/completionTermsApi";
import { CourseCompletionTermsPanel } from "../components/CourseCompletionTermsPanel";
import { type CourseCompletionTermsHistory } from "../model/CourseCompletionTerms";
import {
  defaultTermsDraft,
  termsDecisionPayload,
  termsPayload,
  validateTermsDraft,
  type TermsDraft,
} from "../model/TermsDraft";
import { canDecideTerms, canProposeTerms, openTerms } from "../model/termsDisplay";

export function CourseCompletionTermsRoute({
  session,
  workspace,
}: {
  session: CurrentSession | null;
  workspace: TeacherCourseWorkspaceResponse;
}) {
  const courseId = workspace.course.id;
  const [actionMessage, setActionMessage] = useState<string | null>(null);
  const [actionState, setActionState] = useState<ActionState>("idle");
  const [decisionNote, setDecisionNote] = useState("");
  const [draft, setDraft] = useState<TermsDraft>(defaultTermsDraft);
  const [history, setHistory] = useState<CourseCompletionTermsHistory | null>(null);
  const [loadState, setLoadState] = useState<"idle" | "loading" | "success" | "error">("idle");
  const canPropose = canProposeTerms(workspace);
  const canDecide = canDecideTerms(session, workspace);
  const currentOpenTerms = useMemo(() => openTerms(history?.terms ?? []), [history]);

  const loadTerms = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token) return;
    setLoadState("loading");
    try {
      setHistory(await loadCourseCompletionTerms({ courseId, token }));
      setLoadState("success");
    } catch (error) {
      setActionMessage(normalizeTermsError(error));
      setLoadState("error");
    }
  }, [courseId]);

  useEffect(() => {
    void loadTerms();
  }, [loadTerms]);

  const runAction = useCallback(
    async (action: (token: string) => Promise<unknown>, message: string) => {
      const token = readBrowserSessionToken();
      if (!token) {
        setActionMessage("A signed-in session is required.");
        return;
      }
      setActionState("saving");
      setActionMessage(null);
      try {
        await action(token);
        setActionMessage(message);
        setDraft(defaultTermsDraft);
        setDecisionNote("");
        await loadTerms();
      } catch (error) {
        setActionMessage(normalizeTermsError(error));
      } finally {
        setActionState("idle");
      }
    },
    [loadTerms],
  );

  const submitProposal = useCallback(() => {
    const validation = validateTermsDraft(draft);
    if (validation) {
      setActionMessage(validation);
      return;
    }
    void runAction(
      (token) => submitCourseCompletionTerms({ courseId, payload: termsPayload(draft), token }),
      "Course completion terms proposal submitted.",
    );
  }, [courseId, draft, runAction]);

  const counterProposal = useCallback(() => {
    if (!currentOpenTerms) return;
    const validation = validateTermsDraft(draft);
    if (validation) {
      setActionMessage(validation);
      return;
    }
    void runAction(
      (token) =>
        counterCourseCompletionTerms({
          courseId,
          payload: termsPayload(draft),
          termsId: currentOpenTerms.id,
          token,
        }),
      "Course completion terms counter submitted.",
    );
  }, [courseId, currentOpenTerms, draft, runAction]);

  const decide = useCallback(
    (kind: "accept" | "reject" | "withdraw") => {
      if (!currentOpenTerms) return;
      const payload = termsDecisionPayload(decisionNote);
      const action = {
        accept: acceptCourseCompletionTerms,
        reject: rejectCourseCompletionTerms,
        withdraw: withdrawCourseCompletionTerms,
      }[kind];
      const message = {
        accept: "Course completion terms accepted.",
        reject: "Course completion terms rejected.",
        withdraw: "Course completion terms withdrawn.",
      }[kind];
      void runAction(
        (token) => action({ courseId, payload, termsId: currentOpenTerms.id, token }),
        message,
      );
    },
    [courseId, currentOpenTerms, decisionNote, runAction],
  );

  return (
    <CourseCompletionTermsPanel
      actionMessage={actionMessage}
      actionState={actionState}
      canDecide={canDecide}
      canPropose={canPropose}
      decisionNote={decisionNote}
      draft={draft}
      history={history}
      loadState={loadState}
      onAccept={() => decide("accept")}
      onCounter={counterProposal}
      onDecisionNoteChange={setDecisionNote}
      onDraftChange={setDraft}
      onReject={() => decide("reject")}
      onSubmit={submitProposal}
      onWithdraw={() => decide("withdraw")}
      openTerms={currentOpenTerms}
    />
  );
}

function normalizeTermsError(error: unknown) {
  return normalizeRouteError(error, {
    code: "course_completion_terms_failed",
    message: "Course completion terms request failed.",
    status: 0,
  }).message;
}
