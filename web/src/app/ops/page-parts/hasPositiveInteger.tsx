"use client";
export function hasPositiveInteger(value: string) {
  return /^[1-9]\d*$/.test(value.trim());
}
