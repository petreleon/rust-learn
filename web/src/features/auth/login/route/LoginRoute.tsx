"use client";

import { useLoginController } from "./useLoginController";
import { LoginView } from "../view/LoginView";

export default function LoginRoute() {
  return <LoginView {...useLoginController()} />;
}
