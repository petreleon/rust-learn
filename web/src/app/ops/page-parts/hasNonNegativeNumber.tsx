"use client";
export function hasNonNegativeNumber(value: string) {
  const trimmed = value.trim();
  if (!trimmed) {
    return false;
  }

  const parsed = Number(trimmed);
  return Number.isFinite(parsed) && parsed >= 0;
}
