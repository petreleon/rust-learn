"use client";

import { type ShellNotice } from "@/components/product-shell";
import { type RouteError } from "../model/RouteError";

export function accountNotice(error: RouteError | null): ShellNotice | null {
  if (!error) {
    return null;
  }

  if (error.code === "unauthorized" || error.code === "missing_user") {
    return {
      actionHref: "/login?redirect=/settings/account",
      actionLabel: "Sign in",
      message: "Your stored session is no longer valid. Sign in again to continue.",
      title: "Session expired",
      tone: "error",
    };
  }

  if (error.code === "unverified_email") {
    return {
      actionHref: "/verify-email",
      actionLabel: "Verify email",
      message: "Verify this email address before changing account settings.",
      title: "Email verification required",
      tone: "warn",
    };
  }

  return {
    message: error.message,
    title: "Account status",
    tone: error.code === "timeout" || error.code === "network_error" ? "warn" : "error",
  };
}
