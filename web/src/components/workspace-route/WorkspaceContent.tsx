"use client";

import { type CurrentSession } from "@/lib/session";
import { AdminContent } from "./AdminContent";
import { LearnerContent } from "./LearnerContent";
import { OrganizationContent } from "./OrganizationContent";
import { TeacherContent } from "./TeacherContent";
import { type WorkspaceKind } from "./WorkspaceKind";

export function WorkspaceContent({ kind, session }: { kind: WorkspaceKind; session: CurrentSession }) {
  switch (kind) {
    case "learn":
      return <LearnerContent session={session} />;
    case "teach":
      return <TeacherContent session={session} />;
    case "organizations":
      return <OrganizationContent session={session} />;
    case "admin":
      return <AdminContent session={session} />;
  }
}
