"use client";

import { hasPositiveInteger } from "./hasPositiveInteger";

export function optionalPositiveInteger(value: string) {
  const trimmed = value.trim();
  return hasPositiveInteger(trimmed) ? Number(trimmed) : undefined;
}
