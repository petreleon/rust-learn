import { type CourseCatalogChapter } from "./CourseCatalogChapter";
import { type CourseCatalogItem } from "./CourseCatalogItem";

export type CourseCatalogDetail = {
  chapters: CourseCatalogChapter[];
  course: CourseCatalogItem;
  prerequisites: string[];
};
