export class AuthRequestError extends Error {
  status: number;
  code: string;

  constructor(message: string, status: number, code: string) {
    super(message);
    this.name = "AuthRequestError";
    this.status = status;
    this.code = code;
  }
}
