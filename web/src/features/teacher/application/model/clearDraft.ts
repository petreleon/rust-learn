"use client";

import { DRAFT_STORAGE_KEY } from "./DRAFT_STORAGE_KEY";

export function clearDraft() {
  if (typeof window !== "undefined") {
    window.sessionStorage.removeItem(DRAFT_STORAGE_KEY);
  }
}
