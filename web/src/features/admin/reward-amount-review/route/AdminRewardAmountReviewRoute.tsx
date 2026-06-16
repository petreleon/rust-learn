"use client";

import { AdminRewardAmountReviewView } from "../view/AdminRewardAmountReviewView";
import { useAdminRewardAmountReviewRoute } from "./useAdminRewardAmountReviewRoute";

export function AdminRewardAmountReviewRoute() {
  const route = useAdminRewardAmountReviewRoute();

  return <AdminRewardAmountReviewView route={route} />;
}
