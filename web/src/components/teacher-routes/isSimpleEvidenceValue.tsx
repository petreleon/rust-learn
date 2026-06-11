"use client";
export function isSimpleEvidenceValue(value: unknown) {
  return typeof value === "string" || typeof value === "number" || typeof value === "boolean";
}
