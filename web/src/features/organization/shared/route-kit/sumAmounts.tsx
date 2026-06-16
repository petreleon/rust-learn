"use client";

import { numericAmount } from "./numericAmount";

export function sumAmounts(values: Array<string | number | null | undefined>): number {
  return values.reduce<number>((total, value) => total + numericAmount(value), 0);
}
