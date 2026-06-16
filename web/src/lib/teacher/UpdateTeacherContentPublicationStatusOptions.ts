import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type UpdateTeacherContentPublicationStatusOptions = TeacherRequestOptions & {
  chapterId: number | string;
  contentId: number;
  courseId: number | string;
  publicationStatus: "published" | "unpublished";
};
