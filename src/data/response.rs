#[derive(Debug)]
pub struct Response {
    status: u16,
}

impl Response {
    pub fn empty() -> Self {
        Response { status: 0 }
    }

    pub fn status(&self) -> u16 {
        assert_ne!(self.status, 0);
        self.status
    }
}

impl From<reqwest::blocking::Response> for Response {
    fn from(res: reqwest::blocking::Response) -> Self {
        Response {
            status: res.status().as_u16(),
        }
    }
}
