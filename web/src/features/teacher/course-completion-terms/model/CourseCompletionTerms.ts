export type CourseCompletionTerms = {
  id: number;
  course_id: number;
  teacher_user_id: number;
  organization_id: number | null;
  version: number;
  status: string;
  completion_reward_amount: string;
  max_enrolled_students: number;
  reward_policy_id: number | null;
  proposed_by_user_id: number;
  accepted_by_user_id: number | null;
  accepted_at: string | null;
  activated_at: string | null;
  superseded_at: string | null;
  created_at: string;
  updated_at: string;
};

export type CourseCompletionTermsAuditEvent = {
  id: number;
  terms_id: number;
  course_id: number;
  actor_user_id: number;
  event_type: string;
  previous_status: string | null;
  new_status: string;
  completion_reward_amount: string;
  max_enrolled_students: number;
  note: string | null;
  created_at: string;
};

export type CourseCompletionTermsHistory = {
  active_terms: CourseCompletionTerms | null;
  terms: CourseCompletionTerms[];
  audit_events: CourseCompletionTermsAuditEvent[];
};
