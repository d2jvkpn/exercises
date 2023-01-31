use std::{
    collections::HashMap,
    io::{Result, Write},
};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();

        response.status_code = status_code;

        response.headers = match &headers {
            Some(_h) => headers,
            None => Some(HashMap::from([("Content-Type", "text/html")])),
        };

        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Unknown".into(),
        };

        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        write!(write_stream, "{}", res.to_string())
    }
}

impl<'a> HttpResponse<'a> {
    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let d: HashMap<&str, &str> = self.headers.clone().unwrap_or(HashMap::new());
        d.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join("\r\n")
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }

    pub fn to_string(&self) -> String {
        let body = self.body();

        format!(
            "{} {} {}\r\n{}\r\nContent-Length: {}\r\n\r\n{}",
            self.version(),
            self.status_code(),
            self.status_text(),
            self.headers(),
            body.len(),
            body,
        )
    }
}
