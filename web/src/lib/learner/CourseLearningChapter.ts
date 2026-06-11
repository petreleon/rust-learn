import { type CourseLearningContent } from "./CourseLearningContent";

export type CourseLearningChapter = {
  contents: CourseLearningContent[];
  id: number;
  order: number;
  title: string;
};
