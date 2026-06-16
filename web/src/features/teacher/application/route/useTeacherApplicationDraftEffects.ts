"use client";

import { useEffect, type Dispatch, type SetStateAction } from "react";
import { type ApplicationDraft } from "../model/ApplicationDraft";
import { DRAFT_STORAGE_KEY } from "../model/DRAFT_STORAGE_KEY";
import { isApplicationDraftDirty } from "../model/isApplicationDraftDirty";
import { readDraft } from "../model/readDraft";
import { type SubmitState } from "../model/SubmitState";

export function useTeacherApplicationDraftHydration(
  setDraft: Dispatch<SetStateAction<ApplicationDraft>>,
  setDraftReady: Dispatch<SetStateAction<boolean>>,
) {
  useEffect(() => {
    const timeout = window.setTimeout(() => {
      setDraft(readDraft());
      setDraftReady(true);
    }, 0);
    return () => window.clearTimeout(timeout);
  }, [setDraft, setDraftReady]);
}

export function useTeacherApplicationDraftPersistence(draft: ApplicationDraft, draftReady: boolean) {
  useEffect(() => {
    if (!draftReady) return;
    window.sessionStorage.setItem(DRAFT_STORAGE_KEY, JSON.stringify(draft));
  }, [draft, draftReady]);
}

export function useUnsavedApplicationWarning({
  draft,
  formVisible,
  submitState,
}: {
  draft: ApplicationDraft;
  formVisible: boolean;
  submitState: SubmitState;
}) {
  useEffect(() => {
    const handleBeforeUnload = (event: BeforeUnloadEvent) => {
      if (isApplicationDraftDirty(draft) && formVisible && submitState !== "success") {
        event.preventDefault();
        event.returnValue = "";
      }
    };
    window.addEventListener("beforeunload", handleBeforeUnload);
    return () => window.removeEventListener("beforeunload", handleBeforeUnload);
  }, [draft, formVisible, submitState]);
}
