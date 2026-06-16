"use client";

export function kycStatusTone(status: string): "good" | "neutral" | "warn" {
  if (status === "verified") {
    return "good";
  }
  if (status === "submitted" || status === "under_review" || status === "rejected" || status === "provider_error") {
    return "warn";
  }
  return "neutral";
}
