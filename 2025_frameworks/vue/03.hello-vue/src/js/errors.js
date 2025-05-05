class ApiError extends Error {
  constructor(statusCode, code, msg) {
    super(msg);

    this.name = 'ApiError';
    this.statusCode = statusCode;
    this.code = code;

    Object.setPrototypeOf(this, ApiError.prototype);

    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, ApiError);
    }
  }
}

function biz() {
  throw new ApiError(404, 'not_exists', "account not found");
}

try {
  let ans = biz();
} catch (err) {
  console.log((err instanceof Error) && (err instanceof ApiError));
  console.log(`--> Error: name=${err.name}, statusCode=${err.statusCode}, code=${err.code}, msg=${err.message}`);
}

/*
NetworkError
TypeError: Content-Type != application/json, (SyntaxError) json error
ServerError: statusCode >= 500
UnknownError: 

auth_failed
denied
*/
