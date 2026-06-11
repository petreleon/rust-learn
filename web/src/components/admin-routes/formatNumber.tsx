"use client";
export function formatNumber(value: number) {
  return new Intl.NumberFormat().format(value);
}
