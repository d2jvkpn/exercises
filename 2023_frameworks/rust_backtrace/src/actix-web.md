### Title
---

#### chapter 1
- HttpRequest.extensions_mut().insert(request_id: Uuid)
- HttpRequest.extensions_mut().insert(user_id: i32)
- HttpRequest.extensions().get::<Uuid>()
- HttpRequest.extensions().get(i32)

- impl From<HttpRequest> for MyError {}
