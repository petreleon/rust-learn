import { type TeacherApplicationStatus } from "./TeacherApplicationStatus";

export type TeacherApplicationAuditEvent = {
  actor_user_id: number | null;
  application_id: number;
  created_at: string;
  event_type: string;
  from_status: string | null;
  id: number;
  reason: string | null;
  to_status: TeacherApplicationStatus;
};
