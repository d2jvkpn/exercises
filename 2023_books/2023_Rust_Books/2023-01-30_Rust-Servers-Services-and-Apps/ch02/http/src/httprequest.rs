use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut request = HttpRequest {
            method: Method::Uninitialized,
            version: Version::V1_1,
            resource: Resource::Path("".to_string()),
            headers: HashMap::new(),
            msg_body: "".to_string(),
        };

        let mut req = req.lines();

        (request.method, request.resource, request.version) =
            process_req_line(&req.next().unwrap());

        let mut b = false;

        for line in req {
            if !b && line.len() == 0 {
                b = true;
            }

            if !b {
                let (key, value) = process_header_line(line);
                request.headers.insert(key, value);
                continue;
            }

            if request.msg_body.is_empty() {
                request.msg_body += line;
            } else {
                request.msg_body.push('\n');
                request.msg_body.push_str(line);
            };
        }

        request
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (method.into(), Resource::Path(resource.to_string()), version.into())
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");

    let key = header_items.next().unwrap_or("").to_string();
    let value = header_items.next().unwrap_or("").trim().to_string();

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from(
            r#"GET /greeting HTTP/1.1
Host: localhost
Accept: */*

Hello
World"#,
        );

        let req: HttpRequest = s.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert!(req.headers.contains_key("Host".into()));
        assert!(req.headers.contains_key("Accept".into()));
        assert_eq!(req.msg_body, "Hello\nWorld");
    }
}
