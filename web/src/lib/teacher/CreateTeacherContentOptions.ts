import { type CreateTeacherContentPayload } from "./CreateTeacherContentPayload";
import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type CreateTeacherContentOptions = TeacherRequestOptions & {
  chapterId: number | string;
  courseId: number | string;
  payload: CreateTeacherContentPayload;
};
