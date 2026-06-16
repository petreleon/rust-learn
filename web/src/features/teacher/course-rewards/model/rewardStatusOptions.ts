"use client";

import { type TeacherRewardCandidateStatusFilter } from "@/lib/teacher/TeacherRewardCandidateStatusFilter";

export const rewardStatusOptions: TeacherRewardCandidateStatusFilter[] = [
  "pending_teacher_approval",
  "teacher_approved",
  "teacher_rejected",
  "failed",
  "all",
];
