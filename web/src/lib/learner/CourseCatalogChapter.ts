import { type CourseCatalogContent } from "./CourseCatalogContent";

export type CourseCatalogChapter = {
  contents: CourseCatalogContent[];
  id: number;
  order: number;
  title: string;
};
