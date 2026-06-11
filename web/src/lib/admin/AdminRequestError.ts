export class AdminRequestError extends Error {
  status: number;
  code: string;

  constructor(message: string, status: number, code: string) {
    super(message);
    this.name = "AdminRequestError";
    this.status = status;
    this.code = code;
  }
}
