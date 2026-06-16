"use client";

import { ForgotPasswordView } from "../view/ForgotPasswordView";
import { useForgotPasswordController } from "./useForgotPasswordController";

export default function ForgotPasswordRoute() {
  return <ForgotPasswordView {...useForgotPasswordController()} />;
}
