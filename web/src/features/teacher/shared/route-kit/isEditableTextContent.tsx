"use client";

export function isEditableTextContent(contentType: string) {
  const normalized = contentType.trim().toLowerCase();
  return normalized === "article" || normalized === "text" || normalized.startsWith("text/");
}
