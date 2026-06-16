"use client";
export function isReadableTextContent(contentType: string) {
  const normalized = contentType.trim().toLowerCase();
  return normalized === "text" || normalized === "article" || normalized === "markdown";
}
