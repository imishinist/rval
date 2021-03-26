use reqwest::header::{HeaderName, HeaderValue};

#[derive(Debug, PartialEq)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

impl From<(&HeaderName, &HeaderValue)> for Header {
    fn from(h: (&HeaderName, &HeaderValue)) -> Self {
        let value = unsafe { String::from_utf8_unchecked(h.1.as_bytes().to_owned()) };
        Header {
            name: h.0.to_string(),
            value,
        }
    }
}
