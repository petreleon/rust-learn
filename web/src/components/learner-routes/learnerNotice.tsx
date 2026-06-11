"use client";

import { type ShellNotice } from "@/components/product-shell";
import { loginHref } from "./loginHref";
import { routeConfig } from "./routeConfig";
import { type LearnerRouteKind } from "./LearnerRouteKind";
import { type RouteError } from "./RouteError";

export function learnerNotice(error: RouteError | null, kind: LearnerRouteKind): ShellNotice | null {
  if (!error) {
    return null;
  }

  if (error.code === "unauthorized" || error.code === "missing_user" || error.code === "missing_token") {
    return {
      actionHref: loginHref(routeConfig[kind].href),
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
      message: "Verify this email address before using learner routes.",
      title: "Email verification required",
      tone: "warn",
    };
  }

  return {
    message: error.message,
    title: "Learner route status",
    tone: error.code === "timeout" || error.code === "network_error" ? "warn" : "error",
  };
}
