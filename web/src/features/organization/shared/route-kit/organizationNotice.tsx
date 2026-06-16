"use client";

import { type ShellNotice } from "@/components/product-shell";
import { type RouteError } from "./RouteError";

export function organizationNotice(error: RouteError | null): ShellNotice | null {
  if (!error) {
    return null;
  }

  if (error.code === "unauthorized" || error.code === "missing_user") {
    return {
      actionHref: "/login?redirect=/organizations",
      actionLabel: "Sign in",
      message: "Your stored session is no longer valid. Sign in again to continue.",
      title: "Session expired",
      tone: "error",
    };
  }

  return {
    message: error.message,
    title: "Organization workspace status",
    tone: error.code === "timeout" || error.code === "network_error" ? "warn" : "error",
  };
}
