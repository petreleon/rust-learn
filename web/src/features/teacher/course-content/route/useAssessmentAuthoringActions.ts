"use client";

import { type Dispatch, type FormEvent, type SetStateAction, useState } from "react";
import { type TeacherAssessment } from "@/lib/teacher";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { type ActionState } from "@/features/teacher/shared/route-kit/ActionState";
import { normalizeRouteError } from "@/features/teacher/shared/route-kit/normalizeRouteError";
import {
  assessmentPayloadFromDraft,
  defaultAssessmentDraft,
  draftFromAssessment,
  isAssessmentDraftDirty,
  type AssessmentDraft,
} from "../model/AssessmentDraft";
import {
  createTeacherCourseAssessment,
  updateTeacherCourseAssessment,
} from "../api/assessmentAuthoringApi";

export function useAssessmentAuthoringActions({
  courseId,
  loadContentRoute,
  assessments,
  setActionMessage,
  setActionState,
  setAssessments,
}: {
  assessments: TeacherAssessment[];
  courseId: string;
  loadContentRoute: () => Promise<void>;
  setActionMessage: (message: string | null) => void;
  setActionState: (state: ActionState) => void;
  setAssessments: Dispatch<SetStateAction<TeacherAssessment[]>>;
}) {
  const [assessmentDraft, setAssessmentDraft] = useState<AssessmentDraft>(defaultAssessmentDraft);
  const [editingAssessmentId, setEditingAssessmentId] = useState<number | null>(null);

  function cancelAssessmentEdit() {
    setAssessmentDraft(defaultAssessmentDraft);
    setEditingAssessmentId(null);
    setActionMessage(null);
  }

  function editAssessment(assessment: TeacherAssessment) {
    setAssessmentDraft(draftFromAssessment(assessment));
    setEditingAssessmentId(assessment.id);
    setActionMessage("Editing selected assessment.");
  }

  async function submitAssessment(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readBrowserSessionToken();
    const payload = assessmentPayloadFromDraft(assessmentDraft);
    const validation = assessmentValidation(payload);
    if (!token || validation) {
      setActionMessage(validation || "Sign in again to save this assessment.");
      return;
    }
    setActionState("saving");
    setActionMessage(null);
    try {
      if (editingAssessmentId) {
        await updateTeacherCourseAssessment({ assessmentId: editingAssessmentId, courseId, payload, token });
      } else {
        await createTeacherCourseAssessment({ courseId, payload, token });
      }
      setAssessmentDraft(defaultAssessmentDraft);
      setEditingAssessmentId(null);
      setActionMessage(editingAssessmentId ? "Assessment updated." : "Assessment created.");
      await loadContentRoute();
    } catch (nextError) {
      setActionMessage(normalizeRouteError(nextError).message);
    } finally {
      setActionState("idle");
    }
  }

  return {
    assessmentDraft,
    assessments,
    cancelAssessmentEdit,
    editAssessment,
    editingAssessmentId,
    isAssessmentDraftDirty: isAssessmentDraftDirty(assessmentDraft, editingAssessmentId),
    setAssessmentDraft,
    setAssessments,
    submitAssessment,
  };
}

function assessmentValidation(payload: ReturnType<typeof assessmentPayloadFromDraft>) {
  if (!payload.title) return "Assessment title is required.";
  if (!Number.isFinite(payload.passing_score) || payload.passing_score < 0 || payload.passing_score > 100) {
    return "Passing score must be between 0 and 100.";
  }
  if (!Number.isFinite(payload.max_attempts) || payload.max_attempts < 1) {
    return "Max attempts must be at least 1.";
  }
  if (payload.questions.some((question) => !Number.isFinite(question.points) || question.points < 1)) {
    return "Question points must be at least 1.";
  }
  if (payload.published && !payload.questions.length) {
    return "Published assessments require a question.";
  }
  if (payload.published && payload.questions.some((question) => !question.correct_answer)) {
    return "Published assessments require correct answers.";
  }
  return null;
}
