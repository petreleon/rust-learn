"use client";

import { type CourseLearningContent } from "@/lib/learner";

export function lessonStateCopy(content: CourseLearningContent) {
  switch (content.display_state) {
    case "processing":
      return {
        detail: "This media is still being processed. Check back after the worker finishes.",
        title: "Processing media",
      };
    case "failed_processing":
      return {
        detail: "The media processor failed. Course staff need to retry or replace this upload.",
        title: "Processing failed",
      };
    case "unprocessed_upload":
      return {
        detail: "The upload has not been attached or queued for processing yet.",
        title: "Upload not ready",
      };
    case "uploaded":
      return {
        detail: "The file is uploaded, but this viewer does not yet stream the stored object.",
        title: "Uploaded file",
      };
    case "unavailable":
      return {
        detail: "This lesson has no readable content data yet.",
        title: "Content unavailable",
      };
    default:
      return {
        detail: "This content type is not rendered inline yet.",
        title: "Content preview",
      };
  }
}
