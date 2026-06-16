"use client";

import { type ShellNotice } from "@/components/product-shell";
import { type RouteError } from "../model/RouteError";

export function routeNotice(error: RouteError | null, submitError: RouteError | null): ShellNotice | null {
  if (submitError) {
    return {
      message:
        submitError.code === "conflict"
          ? "Another application state now exists. Review the refreshed status before submitting again."
          : submitError.message,
      title: submitError.code === "conflict" ? "Application state changed" : "Submission failed",
      tone: submitError.code === "conflict" ? "warn" : "error",
    };
  }

  if (!error) {
    return null;
  }

  return {
    actionHref: error.status === 401 ? "/login?redirect=/teach/apply" : undefined,
    actionLabel: error.status === 401 ? "Sign in" : undefined,
    message: error.message,
    title: "Teacher application unavailable",
    tone: "error",
  };
}
