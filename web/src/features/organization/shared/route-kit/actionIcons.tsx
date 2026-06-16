"use client";

import { BookOpen, CreditCard, FileText, Settings, Trophy, UserPlus, Users } from "lucide-react";
import { type ReactNode } from "react";
import { type OrganizationCapabilityKey } from "@/lib/organization";

export const actionIcons: Record<OrganizationCapabilityKey, ReactNode> = {
  courses: <BookOpen size={19} aria-hidden />,
  course_rewards: <Trophy size={19} aria-hidden />,
  member_management: <Users size={19} aria-hidden />,
  members: <Users size={19} aria-hidden />,
  reports: <FileText size={19} aria-hidden />,
  settings: <Settings size={19} aria-hidden />,
  teacher_applications: <UserPlus size={19} aria-hidden />,
  wallet: <CreditCard size={19} aria-hidden />,
};
