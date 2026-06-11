"use client";
export function delegationExpiryLabel(expiresAt: string | null) {
  if (!expiresAt) {
    return "No expiry";
  }

  return `Expires ${expiresAt.replace("T", " ").slice(0, 16)}`;
}
