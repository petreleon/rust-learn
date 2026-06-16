"use client";
export function lifecycleTone(status: string) {
  if (status === "published" || status === "approved") {
    return "good";
  }

  if (status === "archived" || status === "suspended" || status === "needs_changes") {
    return "warn";
  }

  return "neutral";
}
