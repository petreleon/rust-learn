import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type TeacherApplicationStatus } from "./TeacherApplicationStatus";

export type PlatformTeacherApplicationListOptions = AdminRequestOptions & {
  limit?: number;
  offset?: number;
  search?: string | null;
  status?: TeacherApplicationStatus | "" | null;
};
