export type RequestError = Error & {
  code: string;
  status: number;
};

export function isRequestError(error: unknown): error is RequestError {
  return (
    error instanceof Error &&
    "code" in error &&
    "status" in error &&
    typeof error.code === "string" &&
    typeof error.status === "number"
  );
}
