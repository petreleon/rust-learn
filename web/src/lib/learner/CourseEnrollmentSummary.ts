export type CourseEnrollmentSummary = {
  can_request_join: boolean;
  reason: string | null;
  request_id: number | null;
  roles: string[];
  state: string;
};
