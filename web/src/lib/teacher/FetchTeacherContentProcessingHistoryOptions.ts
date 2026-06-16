import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type FetchTeacherContentProcessingHistoryOptions = TeacherRequestOptions & {
  chapterId: number;
  contentId: number;
  courseId: number;
};
