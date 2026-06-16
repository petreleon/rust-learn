"use client";

import { AdminRewardPoliciesView } from "../view/AdminRewardPoliciesView";
import { useAdminRewardPoliciesRoute } from "./useAdminRewardPoliciesRoute";

export function AdminRewardPoliciesRoute() {
  const route = useAdminRewardPoliciesRoute();
  return <AdminRewardPoliciesView route={route} />;
}
