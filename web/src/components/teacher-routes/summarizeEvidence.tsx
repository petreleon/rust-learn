"use client";

import { isSafeEvidenceKey } from "./isSafeEvidenceKey";
import { isSimpleEvidenceValue } from "./isSimpleEvidenceValue";
import { statusLabel } from "./statusLabel";

export function summarizeEvidence(evidence: unknown) {
  if (evidence === null || typeof evidence === "undefined") {
    return "Evidence recorded";
  }

  if (typeof evidence === "string") {
    return evidence.trim() || "Evidence recorded";
  }

  if (typeof evidence === "number" || typeof evidence === "boolean") {
    return String(evidence);
  }

  if (Array.isArray(evidence)) {
    return evidence.length ? `${evidence.length} evidence item${evidence.length === 1 ? "" : "s"}` : "Evidence recorded";
  }

  if (typeof evidence === "object") {
    const entries = Object.entries(evidence as Record<string, unknown>)
      .filter(([key, value]) => isSafeEvidenceKey(key) && isSimpleEvidenceValue(value))
      .slice(0, 3)
      .map(([key, value]) => `${statusLabel(key)}: ${String(value)}`);
    return entries.length ? entries.join(", ") : "Evidence recorded";
  }

  return "Evidence recorded";
}
