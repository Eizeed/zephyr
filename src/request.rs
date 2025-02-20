use std::collections::HashMap;

pub struct HttpRequest {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpRequest {
    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }
}

impl From<&[u8]> for HttpRequest {
    fn from(value: &[u8]) -> Self {
        let str = String::from_utf8_lossy(value);

        let mut split = str.splitn(2, "\r\n");
        let mut start_line = split.next().expect("non standart HTTP request").split(" ");

        let mut split = split.next().expect("non standart HTTP request").splitn(2, "\r\n\r\n");

        let headers_line = split.next().expect("non standart HTTP request");
        let body_line = split.next().expect("non standart HTTP request");

        let method = start_line.next().expect("non standart HTTP request").to_string();
        let path = start_line.next().expect("non standart HTTP request").to_string();
        let version = start_line.next().expect("non standart HTTP request").to_string();

        let mut headers = HashMap::new();
        for header_pair in headers_line.lines() {
            if headers.is_empty() {
                break;
            }

            let mut header_parts = header_pair.split(": ");
            if let (Some(key), Some(val)) = (header_parts.next(), header_parts.next()) {
                headers.insert(key.to_string(), val.to_string());
            }
        }

        Self {
            method,
            path,
            version,
            headers,
            body: body_line.as_bytes().to_vec(),
        }
    }
}
