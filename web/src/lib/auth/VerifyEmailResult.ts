export type VerifyEmailResult = {
  message: string;
  state: "verified" | "already_verified";
};
