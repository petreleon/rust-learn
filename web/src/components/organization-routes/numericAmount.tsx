"use client";
export function numericAmount(value: string | number | null | undefined): number {
  const numericValue = Number(value);
  return Number.isFinite(numericValue) ? numericValue : 0;
}
