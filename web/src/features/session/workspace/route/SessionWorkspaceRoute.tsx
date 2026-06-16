"use client";

import { SessionWorkspaceView } from "../view/SessionWorkspaceView";
import { useSessionWorkspaceRoute } from "./useSessionWorkspaceRoute";

export function SessionWorkspaceRoute() {
  const route = useSessionWorkspaceRoute();

  return <SessionWorkspaceView route={route} />;
}
