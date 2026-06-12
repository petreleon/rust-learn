import { describe, expect, it } from "vitest";
import {
  localEmailPreviewHint,
  registrationSuccessEmailMessage,
  registrationSuccessMessages,
  showLocalEmailPreviewHint,
} from "../registrationSuccessMessages";

describe("registrationSuccessMessages", () => {
  it("keeps production copy user-facing", () => {
    expect(showLocalEmailPreviewHint("production")).toBe(false);
    expect(registrationSuccessMessages("production")).toEqual([registrationSuccessEmailMessage]);
    expect(registrationSuccessMessages("production").join(" ")).not.toContain("API logs");
  });

  it("shows the mock-email hint only for local builds", () => {
    expect(showLocalEmailPreviewHint("development")).toBe(true);
    expect(registrationSuccessMessages("development")).toEqual([registrationSuccessEmailMessage, localEmailPreviewHint]);
  });
});
