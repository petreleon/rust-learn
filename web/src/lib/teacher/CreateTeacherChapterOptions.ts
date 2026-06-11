import { type CreateTeacherChapterPayload } from "./CreateTeacherChapterPayload";
import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type CreateTeacherChapterOptions = TeacherRequestOptions & {
  courseId: number | string;
  payload: CreateTeacherChapterPayload;
};
