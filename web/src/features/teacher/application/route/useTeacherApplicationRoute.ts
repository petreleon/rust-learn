"use client";

import { type FormEvent, useCallback, useEffect, useMemo, useState } from "react";
import { hasTeacherApplicationAccess } from "@/lib/access";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherApplicationSnapshot } from "@/lib/teacher/TeacherApplicationSnapshot";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  loadTeacherApplicationData,
  loadTeacherApplicationSnapshot,
  submitTeacherApplicationDraft,
} from "../api/teacherApplicationApi";
import { type ApplicationDraft } from "../model/ApplicationDraft";
import { type LoadState } from "../model/LoadState";
import { type RouteError } from "../model/RouteError";
import { type SubmitState } from "../model/SubmitState";
import { applicationStatusConfig } from "../model/applicationStatusConfig";
import { clearDraft } from "../model/clearDraft";
import { defaultDraft } from "../model/defaultDraft";
import { emptySnapshot } from "../model/emptySnapshot";
import { isApplicationDraftDirty } from "../model/isApplicationDraftDirty";
import { summarizeWorkspace } from "../model/summarizeWorkspace";
import { validateDraft } from "../model/validateDraft";
import { normalizeApplicationRouteError } from "./normalizeApplicationRouteError";
import {
  useTeacherApplicationDraftHydration,
  useTeacherApplicationDraftPersistence,
  useUnsavedApplicationWarning,
} from "./useTeacherApplicationDraftEffects";

export function useTeacherApplicationRoute() {
  const [draft, setDraft] = useState<ApplicationDraft>(defaultDraft);
  const [draftReady, setDraftReady] = useState(false);
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [showRejectedForm, setShowRejectedForm] = useState(false);
  const [snapshot, setSnapshot] = useState<TeacherApplicationSnapshot>(emptySnapshot);
  const [submitError, setSubmitError] = useState<RouteError | null>(null);
  const [submitState, setSubmitState] = useState<SubmitState>("idle");
  const [validationErrors, setValidationErrors] = useState<string[]>([]);

  const clearRoute = useCallback((nextLoadState: LoadState) => {
    setError(null);
    setLoadState(nextLoadState);
    setSession(null);
    setSnapshot(emptySnapshot);
    setSubmitError(null);
  }, []);

  const loadApplication = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token) {
      setHasToken(false);
      clearRoute("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);
    setSubmitError(null);

    try {
      const next = await loadTeacherApplicationData({ token });
      setSession(next.session);
      setSnapshot(next.snapshot);
      setLoadState("success");
      if (next.snapshot.application?.status !== "rejected") setShowRejectedForm(false);
    } catch (nextError) {
      const routeError = normalizeApplicationRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearBrowserSession();
        setHasToken(false);
      }
      setError(routeError);
      setLoadState("error");
      setSession(null);
      setSnapshot(emptySnapshot);
    }
  }, [clearRoute]);

  useTeacherApplicationDraftHydration(setDraft, setDraftReady);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadApplication(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadApplication]);

  useTeacherApplicationDraftPersistence(draft, draftReady);

  const application = snapshot.application;
  const canSubmitApplication = session ? hasTeacherApplicationAccess(session) : false;
  const formVisible = Boolean(!application || (application.status === "rejected" && showRejectedForm));
  const workspaceSummary = useMemo(() => summarizeWorkspace(session), [session]);
  const selectedOrganization = session?.organizations.find(
    (organization) => String(organization.id) === draft.requestedOrganizationId,
  );
  const selectedCourse = session?.courses.find((course) => String(course.id) === draft.requestedCourseId);
  const statusConfig = applicationStatusConfig(application);

  useUnsavedApplicationWarning({ draft, formVisible, submitState });

  function signOut() {
    if (isApplicationDraftDirty(draft) && formVisible && submitState !== "success") {
      const confirmed = window.confirm("You have unsaved changes in your teacher application. Are you sure you want to sign out?");
      if (!confirmed) return;
    }
    clearBrowserSession();
    setHasToken(false);
    clearRoute("idle");
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readBrowserSessionToken();
    if (!token) {
      setSubmitError({
        code: "missing_token",
        message: "Sign in again before submitting your teacher application.",
        status: 401,
      });
      return;
    }

    const validation = validateDraft(draft, session);
    setValidationErrors(validation.errors);
    if (validation.errors.length) return;

    setSubmitState("submitting");
    setSubmitError(null);
    try {
      await submitTeacherApplicationDraft({ draft, portfolioLinks: validation.portfolioLinks, token });
      setSnapshot(await loadTeacherApplicationSnapshot({ token }));
      setSubmitState("success");
      setShowRejectedForm(false);
      clearDraft();
      setDraft(defaultDraft());
    } catch (nextError) {
      const routeError = normalizeApplicationRouteError(nextError);
      setSubmitError(routeError);
      setSubmitState("error");
      if (routeError.code === "conflict") {
        try {
          setSnapshot(await loadTeacherApplicationSnapshot({ token }));
        } catch {
          // Keep the original submit error visible if refresh also fails.
        }
      }
    }
  }

  return {
    application,
    canSubmitApplication,
    draft,
    error,
    formVisible,
    handleSubmit,
    hasToken,
    loadApplication,
    loadState,
    selectedCourse,
    selectedOrganization,
    session,
    setDraft,
    setShowRejectedForm,
    showRejectedForm,
    signOut,
    snapshot,
    statusConfig,
    submitError,
    submitState,
    validationErrors,
    workspaceSummary,
  };
}

export type TeacherApplicationRouteController = ReturnType<typeof useTeacherApplicationRoute>;
