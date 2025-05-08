class CustomError extends Error {
  constructor(message: string, public code: number) {
    super(message);
    this.name = 'CustomError';
    Object.setPrototypeOf(this, CustomError.prototype);
  }
}

throw new CustomError('a custom error occured', 500);


class ApiError extends Error {
  constructor(
    public statusCode: number,
    public code: string,
    public requestId: string,
    message?: string
  ) {
    super(message || `APIError: statusCode=${statusCode}, code=${code}, requestId=${requestId}`);
    this.name = 'ApiError';
  }
}

throw new ApiError(404,  "not_exists", 'req-123', 'account not found');
