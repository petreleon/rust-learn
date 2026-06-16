"use client";

import { type FormEvent, useState } from "react";
import { clearStoredSessionToken, readStoredSessionToken } from "@/lib/session";
import { type ActionState } from "@/shared/route-state/ActionState";
import { createDraftOrganizationCourse } from "../api/courseApi";
import { validateOrganizationCourseTitle } from "../model/courseCreationModel";
import { normalizeRouteError } from "@/features/organization/shared/route-kit/normalizeRouteError";

export function useOrganizationCourseCreation({
  canCreate,
  loadCourses,
  organizationId,
}: {
  canCreate: boolean;
  loadCourses: () => void;
  organizationId: number | null;
}) {
  const [actionMessage, setActionMessage] = useState<string | null>(null);
  const [actionState, setActionState] = useState<ActionState>("idle");
  const [titleDraft, setTitleDraft] = useState("");

  async function handleCreateCourse(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const validation = validateOrganizationCourseTitle(titleDraft);
    if (validation) {
      setActionMessage(validation);
      return;
    }
    const token = readStoredSessionToken();
    if (!token || !organizationId || !canCreate) return;

    setActionState("saving");
    setActionMessage(null);
    try {
      const created = await createDraftOrganizationCourse({ organizationId, title: titleDraft, token });
      setTitleDraft("");
      setActionMessage(`${created.title} was created as ${created.lifecycle_status}.`);
      void loadCourses();
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) clearStoredSessionToken();
      setActionMessage(routeError.message);
    } finally {
      setActionState("idle");
    }
  }

  return {
    actionMessage,
    actionState,
    handleCreateCourse,
    setTitleDraft,
    titleDraft,
  };
}
