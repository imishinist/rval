use crate::data::Header;

#[derive(Debug)]
pub struct Response {
    status: u16,
    headers: Vec<Header>,
}

impl Response {
    pub fn empty() -> Self {
        Response {
            status: 0,
            headers: Vec::new(),
        }
    }

    pub fn status(&self) -> u16 {
        assert_ne!(self.status, 0);
        self.status
    }

    pub fn headers(&self) -> &[Header] {
        self.headers.as_slice()
    }
}

impl From<reqwest::blocking::Response> for Response {
    fn from(res: reqwest::blocking::Response) -> Self {
        Response {
            status: res.status().as_u16(),
            headers: res.headers().iter().map(|h| h.into()).collect(),
        }
    }
}
