"use client";

import { type LucideIcon } from "lucide-react";
import { type ActiveNav } from "./ActiveNav";

export type NavItem = {
  href: string;
  icon: LucideIcon;
  key: ActiveNav;
  label: string;
};
