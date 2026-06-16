"use client";

import { ServerDeniedNotice } from "./ServerDeniedNotice";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function OpsServerDeniedNotice({
  action,
  actionKey,
  auth,
}: {
  action: string;
  actionKey: string;
  auth: OpsConsoleController["auth"];
}) {
  return auth.hasSessionToken && auth.isServerDenied(actionKey) ? <ServerDeniedNotice action={action} /> : null;
}
