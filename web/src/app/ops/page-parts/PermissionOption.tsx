"use client";
export type PermissionOption = {
  key: string;
  label: string;
  scope: "platform" | "organization" | "course";
};
