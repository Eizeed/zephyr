pub struct HttpResponse {
    status_code: u16,
    body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status_code: u16, body: &[u8]) -> Self {
        Self {
            status_code,
            body: body.to_vec()
        }
    }

    pub fn status_code(status_code: u16) -> Self {
        Self {
            status_code,
            body: vec![],
        }
    }

    pub fn get_status_cide(&self) -> u16 {
        self.status_code
    }

    pub fn get_body(&self) -> &[u8] {
        &self.body
    }
}
