"use client";

import { AdminFraudBlocksView } from "../view/AdminFraudBlocksView";
import { useAdminFraudBlocksRoute } from "./useAdminFraudBlocksRoute";

export function AdminFraudBlocksRoute() {
  const route = useAdminFraudBlocksRoute();

  return <AdminFraudBlocksView route={route} />;
}
