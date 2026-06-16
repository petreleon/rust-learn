export const registrationSuccessTitle = "Registration successful";
export const registrationSuccessEmailMessage = "Check your email to verify your account before signing in.";
export const localEmailPreviewHint = "Local development: the API logs include a mock verification link.";

export function showLocalEmailPreviewHint(nodeEnv = process.env.NODE_ENV) {
  return nodeEnv !== "production";
}

export function registrationSuccessMessages(nodeEnv = process.env.NODE_ENV) {
  return showLocalEmailPreviewHint(nodeEnv)
    ? [registrationSuccessEmailMessage, localEmailPreviewHint]
    : [registrationSuccessEmailMessage];
}
