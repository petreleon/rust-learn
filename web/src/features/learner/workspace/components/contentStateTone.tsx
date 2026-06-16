"use client";
export function contentStateTone(status: string): "bad" | "good" | "neutral" | "warn" {
  if (status === "ready") {
    return "good";
  }
  if (status === "processing" || status === "uploaded" || status === "unprocessed_upload") {
    return "warn";
  }
  if (status === "failed_processing" || status === "unavailable") {
    return "bad";
  }
  return "neutral";
}
