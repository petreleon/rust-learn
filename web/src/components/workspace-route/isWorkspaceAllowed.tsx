"use client";

import { accessSummary } from "@/lib/access";
import { type WorkspaceKind } from "./WorkspaceKind";

export function isWorkspaceAllowed(kind: WorkspaceKind, access: ReturnType<typeof accessSummary>) {
  switch (kind) {
    case "learn":
      return access.learner;
    case "teach":
      return access.teacher;
    case "organizations":
      return access.organization;
    case "admin":
      return access.platformAdmin;
  }
}
