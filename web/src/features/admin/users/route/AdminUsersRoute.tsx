"use client";

import { AdminUsersView } from "../view/AdminUsersView";
import { useAdminUsersRoute } from "./useAdminUsersRoute";

export function AdminUsersRoute() {
  const route = useAdminUsersRoute();
  return <AdminUsersView route={route} />;
}
