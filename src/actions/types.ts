export type Response<T> =
  | {
      data: T;
    }
  | {
      error: string;
    };
