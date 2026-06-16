"use client";

import { type TeacherRouteView } from "../model/TeacherRouteView";
import { TeachingWorkspaceView } from "../view/TeachingWorkspaceView";
import { useTeachingWorkspaceRoute } from "./useTeachingWorkspaceRoute";

export function TeachingWorkspaceRoute({ view }: { view: TeacherRouteView }) {
  const route = useTeachingWorkspaceRoute({ view });

  return <TeachingWorkspaceView route={route} />;
}
