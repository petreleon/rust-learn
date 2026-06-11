"use client";
export function newIdempotencyKey() {
  if (typeof crypto !== "undefined" && "randomUUID" in crypto) {
    return `teacher-application-${crypto.randomUUID()}`;
  }

  return `teacher-application-${Date.now()}-${Math.random().toString(36).slice(2)}`;
}
