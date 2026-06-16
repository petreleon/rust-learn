import { type TeacherContentProcessingJob } from "./TeacherContentProcessingJob";

export type TeacherContentProcessingHistory = {
  content_id: number;
  jobs: TeacherContentProcessingJob[];
  object_key: string | null;
};
