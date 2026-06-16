"use client";
export function enrollmentTone(status: string): "bad" | "good" | "neutral" | "warn" {
  if (status === "enrolled" || status === "available") {
    return "good";
  }
  if (status === "pending" || status === "waitlisted") {
    return "warn";
  }
  if (status === "rejected" || status === "unavailable") {
    return "bad";
  }
  return "neutral";
}
