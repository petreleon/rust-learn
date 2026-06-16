"use client";

import { TeacherApplicationView } from "../view/TeacherApplicationView";
import { useTeacherApplicationRoute } from "./useTeacherApplicationRoute";

export function TeacherApplicationRoute() {
  const route = useTeacherApplicationRoute();

  return <TeacherApplicationView route={route} />;
}
