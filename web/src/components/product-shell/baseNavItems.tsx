"use client";

import { Home, Settings } from "lucide-react";
import { type NavItem } from "./NavItem";

export const baseNavItems: NavItem[] = [
  { key: "session", href: "/session", label: "Workspace", icon: Home },
  { key: "account", href: "/settings/account", label: "Account", icon: Settings },
];
