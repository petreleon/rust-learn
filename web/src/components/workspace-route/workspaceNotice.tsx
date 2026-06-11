"use client";

import { type ShellNotice } from "../product-shell";
import { type WorkspaceKind } from "./WorkspaceKind";

export function workspaceNotice(
  error: { code: string; message: string } | null,
  kind: WorkspaceKind,
): ShellNotice | null {
  if (!error) {
    return null;
  }

  if (error.code === "unauthorized" || error.code === "missing_user") {
    return {
      actionHref: `/login?redirect=/${kind}`,
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
