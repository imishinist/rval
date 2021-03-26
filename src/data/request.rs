use crate::data::Scenario;

pub struct Request {
    url: String,
}

impl Request {
    pub fn new(url: String) -> Self {
        Request { url }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

impl From<&Scenario> for Request {
    fn from(s: &Scenario) -> Self {
        Request::new(s.url().clone())
    }
}
