"use client";

import { useCallback, useEffect, useState } from "react";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { normalizeRouteError } from "../../components/normalizeRouteError";
import {
  loadCourseAssessmentSnapshot,
  submitCourseAssessmentAttempt,
  type CourseAssessmentSnapshot,
} from "../api/courseAssessmentApi";
import { emptyAssessmentDraft, type AssessmentDraft } from "../model/assessmentState";

type LoadState = "idle" | "loading" | "success" | "error";

export function useCourseAssessments(courseId: number) {
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [snapshot, setSnapshot] = useState<CourseAssessmentSnapshot | null>(null);
  const [drafts, setDrafts] = useState<Record<number, AssessmentDraft>>({});
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [submittingAssessmentId, setSubmittingAssessmentId] = useState<number | null>(null);

  const loadAssessments = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token) {
      setLoadState("idle");
      setSnapshot(null);
      return;
    }

    setLoadState("loading");
    setError(null);
    try {
      const nextSnapshot = await loadCourseAssessmentSnapshot({ courseId, token });
      setSnapshot(nextSnapshot);
      setDrafts(
        Object.fromEntries(
          nextSnapshot.assessments.map((assessment) => [
            assessment.id,
            emptyAssessmentDraft(assessment),
          ]),
        ),
      );
      setLoadState("success");
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401) clearBrowserSession();
      setError(requestError.message);
      setLoadState("error");
    }
  }, [courseId]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadAssessments(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadAssessments]);

  function updateAnswer(assessmentId: number, questionId: number, value: string) {
    setDrafts((current) => ({
      ...current,
      [assessmentId]: {
        ...current[assessmentId],
        [questionId]: value,
      },
    }));
  }

  async function submitAssessment(assessmentId: number) {
    const token = readBrowserSessionToken();
    const assessment = snapshot?.assessments.find((item) => item.id === assessmentId);
    if (!token || !assessment) return;

    setSubmittingAssessmentId(assessmentId);
    setNotice(null);
    try {
      const result = await submitCourseAssessmentAttempt({
        answers: drafts[assessmentId] || {},
        assessmentId,
        courseId,
        token,
      });
      setSnapshot((current) => current && {
        ...current,
        attemptsByAssessmentId: {
          ...current.attemptsByAssessmentId,
          [assessmentId]: [result.attempt, ...(current.attemptsByAssessmentId[assessmentId] || [])],
        },
      });
      setDrafts((current) => ({ ...current, [assessmentId]: emptyAssessmentDraft(assessment) }));
      setNotice(result.passed ? "Assessment passed." : "Attempt submitted.");
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401) clearBrowserSession();
      setNotice(requestError.message);
    } finally {
      setSubmittingAssessmentId(null);
    }
  }

  return {
    drafts,
    error,
    loadAssessments,
    loadState,
    notice,
    snapshot,
    submittingAssessmentId,
    submitAssessment,
    updateAnswer,
  };
}
