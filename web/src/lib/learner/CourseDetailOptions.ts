import { type LearnerRequestOptions } from "./LearnerRequestOptions";

export type CourseDetailOptions = LearnerRequestOptions & {
  courseId: number;
};
