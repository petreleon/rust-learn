"use client";
export function joinRequestTone(status: string) {
  if (status === "approved") {
    return "good";
  }

  if (status === "pending" || status === "waitlisted") {
    return "warn";
  }

  return "neutral";
}
