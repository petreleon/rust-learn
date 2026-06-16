import { type TeacherRequestOptions } from "./TeacherRequestOptions";
import { type UpdateTeacherContentPayload } from "./UpdateTeacherContentPayload";

export type UpdateTeacherContentOptions = TeacherRequestOptions & {
  chapterId: number | string;
  contentId: number;
  courseId: number | string;
  payload: UpdateTeacherContentPayload;
};
