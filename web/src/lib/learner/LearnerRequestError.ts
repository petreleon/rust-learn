export class LearnerRequestError extends Error {
  status: number;
  code: string;

  constructor(message: string, status: number, code: string) {
    super(message);
    this.name = "LearnerRequestError";
    this.status = status;
    this.code = code;
  }
}
