import { type TeacherApplication } from "./TeacherApplication";
import { type TeacherApplicationAuditEvent } from "./TeacherApplicationAuditEvent";

export type TeacherApplicationSnapshot = {
  application: TeacherApplication | null;
  audit_events: TeacherApplicationAuditEvent[];
};
