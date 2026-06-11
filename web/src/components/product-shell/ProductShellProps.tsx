"use client";

import { type ReactNode } from "react";
import { type CurrentSession } from "@/lib/session";
import { type ActiveNav } from "./ActiveNav";
import { type Breadcrumb } from "./Breadcrumb";
import { type ShellNotice } from "./ShellNotice";

export type ProductShellProps = {
  activeNav: ActiveNav;
  breadcrumbs?: Breadcrumb[];
  children: ReactNode;
  description: string;
  eyebrow: string;
  isSignedIn?: boolean;
  notice?: ShellNotice | null;
  onSignOut?: () => void;
  session?: CurrentSession | null;
  statusItems?: ReactNode;
  title: string;
};
