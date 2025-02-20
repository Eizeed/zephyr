use crate::status::StatusCode;

pub struct HttpResponse {
    status_code: StatusCode,
    body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status_code: StatusCode, body: &[u8]) -> Self {
        Self {
            status_code,
            body: body.to_vec()
        }
    }

    pub fn status_code(status_code: StatusCode) -> Self {
        Self {
            status_code,
            body: vec![],
        }
    }

    pub fn get_status_code(&self) -> &StatusCode {
        &self.status_code
    }

    pub fn get_body(&self) -> &[u8] {
        &self.body
    }
}
