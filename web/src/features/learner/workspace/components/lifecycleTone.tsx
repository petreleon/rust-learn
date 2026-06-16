"use client";
export function lifecycleTone(status: string): "bad" | "good" | "neutral" | "warn" {
  if (status === "published") return "good";
  if (status === "suspended") return "bad";
  if (status === "archived") return "warn";
  if (status === "submitted" || status === "needs_changes") return "warn";
  return "neutral";
}
