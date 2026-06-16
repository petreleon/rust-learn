"use client";

import { AdminWalletsView } from "../view/AdminWalletsView";
import { useAdminWalletsRoute } from "./useAdminWalletsRoute";

export function AdminWalletsRoute() {
  const route = useAdminWalletsRoute();
  return <AdminWalletsView route={route} />;
}
