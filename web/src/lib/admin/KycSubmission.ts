export type KycSubmission = {
  id: number;
  user_id: number;
  status: string;
  legal_name: string;
  country_code: string;
  document_type: string;
  document_last4: string | null;
  evidence_reference: string | null;
  provider_reference: string | null;
  reviewer_user_id: number | null;
  rejection_reason: string | null;
  submitted_at: string;
  reviewed_at: string | null;
  created_at: string;
  updated_at: string;
};
