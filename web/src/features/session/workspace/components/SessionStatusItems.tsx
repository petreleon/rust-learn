"use client";

import { Building2, ShieldCheck } from "lucide-react";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "../model/LoadState";
import { StatusPill } from "./StatusPill";

export function SessionStatusItems({
  loadState,
  session,
  workspaceCount,
}: {
  loadState: LoadState;
  session: CurrentSession | null;
  workspaceCount: number;
}) {
  return (
    <>
      <StatusPill
        icon={<ShieldCheck size={16} aria-hidden />}
        label={loadState === "loading" ? "Resolving session" : session ? "Verified session" : "No active session"}
        tone={session ? "good" : loadState === "error" ? "warn" : "neutral"}
      />
      <StatusPill icon={<Building2 size={16} aria-hidden />} label={`${workspaceCount} workspaces`} tone="neutral" />
    </>
  );
}
