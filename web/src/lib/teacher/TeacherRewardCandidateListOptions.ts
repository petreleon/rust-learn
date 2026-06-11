import { type TeacherRequestOptions } from "./TeacherRequestOptions";
import { type TeacherRewardCandidateStatusFilter } from "./TeacherRewardCandidateStatusFilter";

export type TeacherRewardCandidateListOptions = TeacherRequestOptions & {
  courseId: number | string;
  limit?: number;
  offset?: number;
  status?: TeacherRewardCandidateStatusFilter | string;
  studentUserId?: number;
};
