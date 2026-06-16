"use client";
export function prettyBody(value: string) {
  if (!value) {
    return "";
  }
  try {
    return JSON.stringify(JSON.parse(value), null, 2);
  } catch {
    return value;
  }
}
