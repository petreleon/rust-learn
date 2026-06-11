export class SessionRequestError extends Error {
  status: number;
  code: string;

  constructor(message: string, status: number, code: string) {
    super(message);
    this.name = "SessionRequestError";
    this.status = status;
    this.code = code;
  }
}
