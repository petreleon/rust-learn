export class TeacherRequestError extends Error {
  status: number;
  code: string;

  constructor(message: string, status: number, code: string) {
    super(message);
    this.name = "TeacherRequestError";
    this.status = status;
    this.code = code;
  }
}
