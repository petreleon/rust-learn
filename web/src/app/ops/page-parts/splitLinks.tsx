"use client";
export function splitLinks(value: string) {
  const links = value
    .split(/[\n,]/)
    .map((link) => link.trim())
    .filter(Boolean);
  return links.length > 0 ? links : undefined;
}
