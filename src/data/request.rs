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
