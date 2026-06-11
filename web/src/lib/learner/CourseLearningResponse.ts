import { type CourseCatalogItem } from "./CourseCatalogItem";
import { type CourseLearningChapter } from "./CourseLearningChapter";

export type CourseLearningResponse = {
  active_content_id: number | null;
  chapters: CourseLearningChapter[];
  course: CourseCatalogItem;
  progress_supported: boolean;
};
