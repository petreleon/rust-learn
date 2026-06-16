"use client";

import { Search } from "lucide-react";

export const routeConfig = {
  dashboard: {
    description: "Continue learning, inspect rewards, and keep your wallet ready.",
    href: "/learn",
    title: "Learner dashboard",
  },
  courses: {
    description: "Search courses, review enrollment state, and request access.",
    href: "/courses",
    title: "Courses",
  },
  rewards: {
    description: "Reward history with human status, wallet credit, and course context.",
    href: "/rewards",
    title: "Rewards",
  },
  wallet: {
    description: "Wallet link state, balance, and reward-credit readiness.",
    href: "/wallet",
    title: "Wallet",
  },
} as const;
