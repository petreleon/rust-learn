"use client";

import { type FraudBlockItem, type PlatformFraudDashboard } from "@/lib/admin";

export function targetLabel(block: FraudBlockItem | PlatformFraudDashboard["active_blocks"][number]) {
  if (block.teacher_user_id) {
    return `Teacher user ${block.teacher_user_id}`;
  }
  if (block.organization_id) {
    return `Organization ${block.organization_id}`;
  }
  if (block.course_id) {
    return `Course ${block.course_id}`;
  }
  if (block.reward_policy_id) {
    return `Reward policy ${block.reward_policy_id}`;
  }
  return "Scope target unavailable";
}
