export class ActionResult<T> {
  data?: T;
  error?: string;

  constructor(data?: T, error?: string) {
    this.data = data;
    this.error = error;
  }

  static ok<T>(data: T) {
    return new ActionResult<T>(data, undefined);
  }
  static error<T>(error: string) {
    return new ActionResult<T>(undefined, error);
  }

  isOk() {
    return !this.error;
  }

  isError() {
    return !!this.error;
  }
}
