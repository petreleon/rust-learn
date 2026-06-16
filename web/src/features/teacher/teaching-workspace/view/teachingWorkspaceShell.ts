import { type ShellNotice } from "@/components/product-shell";
import { type RouteError } from "@/shared/route-state/RouteError";
import { type TeacherRouteView } from "../model/TeacherRouteView";

export function teachingRouteNotice(error: RouteError | null): ShellNotice | null {
  if (!error) return null;

  return {
    actionHref: error.status === 401 ? "/login?redirect=/teach" : undefined,
    actionLabel: error.status === 401 ? "Sign in" : undefined,
    message: error.message,
    title: "Teaching workspace unavailable",
    tone: "error",
  };
}

export function teacherBreadcrumbs(view: TeacherRouteView) {
  if (view === "courses") {
    return [
      { href: "/session", label: "Workspace" },
      { href: "/teach", label: "Teach" },
      { label: "Courses" },
    ];
  }
  return [
    { href: "/session", label: "Workspace" },
    { label: "Teach" },
  ];
}

export function teacherDescription(view: TeacherRouteView) {
  if (view === "courses") {
    return "Teaching courses, lifecycle state, enrollment pressure, reward review queues, and scoped actions.";
  }
  return "Teaching work, application state, course health, enrollment pressure, and reward review queues.";
}
