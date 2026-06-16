"use client";

import { type ShellNotice } from "@/components/product-shell";
import { type RouteError } from "../model/RouteError";

export function sessionNotice(error: RouteError | null): ShellNotice | null {
  if (!error) {
    return null;
  }

  if (error.code === "unauthorized" || error.code === "missing_user") {
    return {
      actionHref: "/login?redirect=/session",
      actionLabel: "Sign in",
      message: "Your stored session is no longer valid. Sign in again to continue.",
      title: "Session expired",
      tone: "error",
    };
  }

  return {
    message: error.message,
    title: "Workspace status",
    tone: error.code === "timeout" || error.code === "network_error" ? "warn" : "error",
  };
}
