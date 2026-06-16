import { type CourseCatalogResponse } from "@/lib/learner";

export function catalogVisibleRange(catalog: CourseCatalogResponse | null) {
  if (!catalog) {
    return {
      detail: null,
      label: "0 shown",
    };
  }

  const shown = catalog.courses.length;
  const start = shown ? catalog.offset + 1 : 0;
  const end = catalog.offset + shown;
  const label = shown === catalog.total ? `${shown} shown` : `${shown} of ${catalog.total} shown`;
  const detail = end < catalog.total
    ? `Showing ${start}-${end} of ${catalog.total} matches. Use search or filters to narrow the catalog.`
    : null;

  return { detail, label };
}
