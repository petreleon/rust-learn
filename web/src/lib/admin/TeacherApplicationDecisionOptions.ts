import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type TeacherApplicationStatus } from "./TeacherApplicationStatus";

export type TeacherApplicationDecisionOptions = AdminRequestOptions & {
  applicationId: number;
  decisionReason?: string | null;
  status: Extract<TeacherApplicationStatus, "approved" | "needs_changes" | "rejected">;
};
