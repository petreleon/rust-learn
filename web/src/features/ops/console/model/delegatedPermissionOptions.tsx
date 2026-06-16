"use client";
export const delegatedPermissionOptions: Array<{ key: string; scopes: string[] }> = [
  { key: "APPROVE_REWARD_AMOUNT", scopes: ["platform"] },
  { key: "EXECUTE_REWARD_PAYOUT", scopes: ["platform"] },
  { key: "VIEW_REWARD_AUDIT", scopes: ["platform"] },
  { key: "MANAGE_REWARD_FRAUD_BLOCKS", scopes: ["platform"] },
  { key: "BLOCK_REWARD_TEACHER", scopes: ["platform"] },
  { key: "BLOCK_REWARD_ORGANIZATION", scopes: ["platform"] },
  { key: "SUBMIT_ORG_COURSE_REWARD_EVENT", scopes: ["organization"] },
  { key: "VIEW_ORG_REWARD_REPORTS", scopes: ["organization"] },
  { key: "MANAGE_ORG_REWARD_BUDGET", scopes: ["organization"] },
  { key: "SUBMIT_COURSE_REWARD_EVENT", scopes: ["course"] },
  { key: "CREATE_REWARDABLE_COURSE_EVENT", scopes: ["course"] },
  { key: "APPROVE_STUDENT_REWARD_CANDIDATE", scopes: ["course"] },
  { key: "VIEW_COURSE_REWARD_STATUS", scopes: ["course"] },
  { key: "GRADE_REWARDABLE_ASSESSMENT", scopes: ["course"] },
  { key: "MANAGE_COURSE_REWARD_RULES", scopes: ["course"] },
];
