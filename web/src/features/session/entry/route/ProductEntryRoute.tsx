"use client";

import { ProductEntryView } from "../view/ProductEntryView";
import { useProductEntryRoute } from "./useProductEntryRoute";

export default function ProductEntryRoute() {
  useProductEntryRoute();
  return <ProductEntryView />;
}
