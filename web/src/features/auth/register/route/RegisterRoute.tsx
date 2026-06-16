"use client";

import { RegisterView } from "../view/RegisterView";
import { useRegisterController } from "./useRegisterController";

export default function RegisterRoute() {
  return <RegisterView {...useRegisterController()} />;
}
