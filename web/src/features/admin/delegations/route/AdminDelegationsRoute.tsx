"use client";

import { AdminDelegationsView } from "../view/AdminDelegationsView";
import { useAdminDelegationsRoute } from "./useAdminDelegationsRoute";

export function AdminDelegationsRoute() {
  const route = useAdminDelegationsRoute();
  return <AdminDelegationsView route={route} />;
}
