import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type FetchUploadUrlOptions = TeacherRequestOptions & {
  chapterId: number;
  contentType: string;
  courseId: number;
  filename: string;
};
