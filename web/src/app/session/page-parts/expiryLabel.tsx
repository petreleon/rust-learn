"use client";
export function expiryLabel(expiresAt: string) {
  return `Expires ${expiresAt.replace("T", " ").slice(0, 16)}`;
}
