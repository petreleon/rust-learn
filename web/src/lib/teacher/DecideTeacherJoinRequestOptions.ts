import { type DecideTeacherJoinRequestPayload } from "./DecideTeacherJoinRequestPayload";
import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type DecideTeacherJoinRequestOptions = TeacherRequestOptions & {
  courseId: number | string;
  payload: DecideTeacherJoinRequestPayload;
  requestId: number | string;
};
