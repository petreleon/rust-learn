"use client";

import { type TeacherRewardCandidateDecisionStatus } from "@/lib/teacher";

export type RewardDecisionDraft = {
  reason: string;
  status: TeacherRewardCandidateDecisionStatus;
};
