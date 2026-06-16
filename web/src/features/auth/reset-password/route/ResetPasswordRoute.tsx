"use client";

import { ResetPasswordView } from "../view/ResetPasswordView";
import { useResetPasswordController } from "./useResetPasswordController";

export default function ResetPasswordRoute() {
  return <ResetPasswordView {...useResetPasswordController()} />;
}
