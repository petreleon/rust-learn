import { type DecideTeacherRewardCandidatePayload } from "./DecideTeacherRewardCandidatePayload";
import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type DecideTeacherRewardCandidateOptions = TeacherRequestOptions & {
  candidateId: number | string;
  courseId: number | string;
  payload: DecideTeacherRewardCandidatePayload;
};
