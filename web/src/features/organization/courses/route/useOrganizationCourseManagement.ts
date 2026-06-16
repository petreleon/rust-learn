"use client";

import { type FormEvent, useState } from "react";
import { type OrganizationCourseListItem } from "@/lib/organization";
import { readStoredSessionToken } from "@/lib/session";
import { type ActionState } from "@/shared/route-state/ActionState";
import { normalizeRouteError } from "@/features/organization/shared/route-kit/normalizeRouteError";
import {
  saveOrganizationCourseLifecycle,
  saveOrganizationCourseSettings,
} from "../api/courseApi";
import {
  canManageOrganizationCourse,
  courseManagementDraft,
  courseManagementValidation,
  type CourseManagementDraft,
} from "../model/courseManagementModel";

export function useOrganizationCourseManagement({ loadCourses }: { loadCourses: () => Promise<void> }) {
  const [actionMessage, setActionMessage] = useState<string | null>(null);
  const [actionState, setActionState] = useState<ActionState>("idle");
  const [draft, setDraft] = useState<CourseManagementDraft>({ title: "" });
  const [lifecycleDraft, setLifecycleDraft] = useState("");
  const [selectedCourseId, setSelectedCourseId] = useState<number | null>(null);

  function selectCourse(course: OrganizationCourseListItem | null) {
    setActionMessage(null);
    setSelectedCourseId(course?.id ?? null);
    setDraft(course ? courseManagementDraft(course) : { title: "" });
    setLifecycleDraft(course?.lifecycle_status ?? "");
  }

  async function runAction(action: (token: string, courseId: number) => Promise<unknown>, success: string) {
    const token = readStoredSessionToken();
    if (!token || !selectedCourseId) return;
    setActionMessage(null);
    setActionState("saving");
    try {
      await action(token, selectedCourseId);
      setActionMessage(success);
      await loadCourses();
    } catch (nextError) {
      setActionMessage(normalizeRouteError(nextError).message);
    } finally {
      setActionState("idle");
    }
  }

  async function handleSettingsSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const validation = courseManagementValidation(draft);
    if (validation) {
      setActionMessage(validation);
      return;
    }
    await runAction(
      (token, courseId) => saveOrganizationCourseSettings({ courseId, draft, token }),
      "Course settings updated.",
    );
  }

  async function handleLifecycleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!lifecycleDraft) {
      setActionMessage("Choose a lifecycle status first.");
      return;
    }
    await runAction(
      (token, courseId) => saveOrganizationCourseLifecycle({ courseId, status: lifecycleDraft, token }),
      "Course lifecycle updated.",
    );
  }

  return {
    actionMessage,
    actionState,
    canManageOrganizationCourse,
    draft,
    handleLifecycleSubmit,
    handleSettingsSubmit,
    lifecycleDraft,
    selectCourse,
    selectedCourseId,
    setDraft,
    setLifecycleDraft,
  };
}

export type OrganizationCourseManagementController = ReturnType<typeof useOrganizationCourseManagement>;
