"use client";

import { type ActiveNav } from "../product-shell/ActiveNav";
import { type WorkspaceKind } from "./WorkspaceKind";

export type WorkspaceConfig = {
  activeNav: ActiveNav;
  deniedSignals: string[];
  description: string;
  eyebrow: string;
  title: string;
};

export const workspaceConfig: Record<WorkspaceKind, WorkspaceConfig> = {
  learn: {
    activeNav: "learn",
    deniedSignals: ["A verified product session"],
    description: "Courses, learning access, reward progress, and wallet readiness.",
    eyebrow: "Learner",
    title: "Learner workspace",
  },
  teach: {
    activeNav: "teach",
    deniedSignals: [
      "An approved course teaching permission",
      "A delegated course teaching permission",
      "Teacher application capability",
    ],
    description: "Teaching scopes, course permissions, and application status signals.",
    eyebrow: "Teacher",
    title: "Teaching workspace",
  },
  organizations: {
    activeNav: "organizations",
    deniedSignals: [
      "Organization membership",
      "Organization-scoped view or management permission",
      "Delegated organization permission",
    ],
    description: "Organization memberships, scoped permissions, reports, and wallet readiness.",
    eyebrow: "Organization",
    title: "Organization workspace",
  },
  admin: {
    activeNav: "admin",
    deniedSignals: [
      "Platform review permission",
      "Platform reward, fraud, export, wallet, or settings permission",
      "Active delegated platform permission",
    ],
    description: "Platform review, reward, fraud, export, wallet, and audit scopes.",
    eyebrow: "Platform",
    title: "Admin workspace",
  },
};
