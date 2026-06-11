"use client";
export function normalizeRoot(root: string) {
  const trimmed = root.trim();
  return trimmed.endsWith("/") ? trimmed.slice(0, -1) : trimmed;
}
