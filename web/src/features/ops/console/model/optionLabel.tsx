"use client";
export function optionLabel(value: string) {
  return value
    .split("_")
    .filter(Boolean)
    .map((word) => `${word.charAt(0).toUpperCase()}${word.slice(1).toLowerCase()}`)
    .join(" ");
}
