"use client";

import { AdminDashboardView } from "../view/AdminDashboardView";
import { useAdminDashboardRoute } from "./useAdminDashboardRoute";

export function AdminDashboardRoute() {
  const route = useAdminDashboardRoute();
  return <AdminDashboardView route={route} />;
}
