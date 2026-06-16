export const passwordPolicyChecks = [
  {
    key: "length",
    label: "At least 12 characters",
    test: (password: string) => password.length >= 12,
  },
  {
    key: "lower",
    label: "Lowercase letter",
    test: (password: string) => /[a-z]/.test(password),
  },
  {
    key: "upper",
    label: "Uppercase letter",
    test: (password: string) => /[A-Z]/.test(password),
  },
  {
    key: "number",
    label: "Number",
    test: (password: string) => /[0-9]/.test(password),
  },
  {
    key: "symbol",
    label: "Symbol",
    test: (password: string) => /[^A-Za-z0-9]/.test(password),
  },
  {
    key: "bcrypt",
    label: "At most 71 UTF-8 bytes",
    test: (password: string) => new TextEncoder().encode(password).length <= 71,
  },
];

export type PasswordPolicyCheck = (typeof passwordPolicyChecks)[number];
export type PasswordPolicyResult = PasswordPolicyCheck & { met: boolean };

export function evaluatePasswordPolicy(password: string) {
  return passwordPolicyChecks.map((check) => ({ ...check, met: check.test(password) }));
}
