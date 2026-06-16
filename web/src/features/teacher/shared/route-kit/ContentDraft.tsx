"use client";
export type ContentDraft = {
  chapterId: string;
  contentType: string;
  data: string;
  file: File | null;
  filename: string;
  order: string;
  uploadKind: "text" | "file";
};
