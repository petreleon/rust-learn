"use client";

import { hasText } from "./hasText";

export function hasAnyText(values: string[]) {
  return values.some(hasText);
}
