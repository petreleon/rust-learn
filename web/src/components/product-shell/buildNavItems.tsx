"use client";

import { BriefcaseBusiness, Home } from "lucide-react";
import { accessSummary } from "@/lib/access";
import { type CurrentSession } from "@/lib/session";
import { baseNavItems } from "./baseNavItems";
import { type NavItem } from "./NavItem";

export function buildNavItems(session?: CurrentSession | null) {
  if (!session) {
    return baseNavItems;
  }

  const access = accessSummary(session);
  return [
    baseNavItems[0],
    access.learner
      ? { key: "learn", href: "/learn", label: "Learn", icon: Home }
      : null,
    access.teacher
      ? { key: "teach", href: "/teach", label: "Teach", icon: BriefcaseBusiness }
      : null,
    access.organization
      ? { key: "organizations", href: "/organizations", label: "Organizations", icon: BriefcaseBusiness }
      : null,
    access.platformAdmin
      ? { key: "admin", href: "/admin", label: "Admin", icon: BriefcaseBusiness }
      : null,
    baseNavItems[1],
  ].filter((item): item is NavItem => Boolean(item));
}
