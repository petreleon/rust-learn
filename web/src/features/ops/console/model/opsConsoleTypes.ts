"use client";

import { type Dispatch, type SetStateAction } from "react";
import { type HttpMethod } from "./HttpMethod";

export type SendOpsApi = (label: string, path: string, method?: HttpMethod, body?: unknown) => Promise<void>;

export type HasOpsPermission = (permission: string) => boolean;

export type TeacherApplicationDraft = {
  requested_scope: string;
  requested_organization_id: string;
  requested_course_id: string;
  experience_summary: string;
  organization_sponsor_id: string;
  portfolio_links: string;
};

export type TeacherDecisionDraft = {
  application_id: string;
  status: string;
  decision_reason: string;
};

export type TeacherRewardDecisionDraft = {
  status: string;
  decision_reason: string;
};

export type AmountDecisionDraft = {
  status: string;
  approved_amount: string;
  decision_reason: string;
};

export type FraudBlockDraft = {
  scope_type: string;
  teacher_user_id: string;
  organization_id: string;
  course_id: string;
  reward_policy_id: string;
  reason: string;
  evidence_reference: string;
};

export type DelegationDraft = {
  grantee_user_id: string;
  permission: string;
  scope_type: string;
  organization_id: string;
  course_id: string;
  reason: string;
  expires_at: string;
};

export type DraftSetter<T> = Dispatch<SetStateAction<T>>;

export type ScopePermission = {
  allowed: boolean;
  detail: string;
  permissionTitle: string;
};
