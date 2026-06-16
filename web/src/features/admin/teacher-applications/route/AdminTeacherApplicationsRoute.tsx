"use client";

import { AdminTeacherApplicationsView } from "../view/AdminTeacherApplicationsView";
import { useAdminTeacherApplicationsRoute } from "./useAdminTeacherApplicationsRoute";

export function AdminTeacherApplicationsRoute() {
  const route = useAdminTeacherApplicationsRoute();

  return <AdminTeacherApplicationsView route={route} />;
}
