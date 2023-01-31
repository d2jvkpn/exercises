use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{HttpRequest, Method, Resource};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) {
        if req.method != Method::Get {
            let resp = PageNotFoundHandler::handle(&req);
            let _ = resp.send_response(stream);
            return;
        }

        let Resource::Path(p) = &req.resource;

        match p.as_str() {
            v if v.starts_with("/api") => {
                let resp = WebServiceHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
            _ => {
                let resp = StaticPageHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
