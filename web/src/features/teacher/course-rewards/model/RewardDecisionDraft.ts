"use client";

import { type TeacherRewardCandidateDecisionStatus } from "@/lib/teacher/TeacherRewardCandidateDecisionStatus";

export type RewardDecisionDraft = {
  reason: string;
  status: TeacherRewardCandidateDecisionStatus;
};
