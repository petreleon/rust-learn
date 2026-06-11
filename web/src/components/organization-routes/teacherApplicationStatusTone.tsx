"use client";
export function teacherApplicationStatusTone(status: string): "good" | "neutral" | "warn" {
  if (status === "approved") {
    return "good";
  }

  if (status === "needs_changes" || status === "rejected") {
    return "warn";
  }

  return "neutral";
}
