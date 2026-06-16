"use client";

import { useOpsConsoleController } from "./useOpsConsoleController";
import { OpsConsoleView } from "../view/OpsConsoleView";

export default function OpsConsoleRoute() {
  const controller = useOpsConsoleController();
  return <OpsConsoleView controller={controller} />;
}
