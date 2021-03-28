use crate::data::Header;

#[derive(Debug, Clone)]
pub struct Spec {
    status: Option<u16>,
    header: Option<Header>,
}

impl Spec {
    pub fn status(&self) -> Option<u16> {
        self.status
    }

    pub fn header(&self) -> Option<&Header> {
        self.header.as_ref()
    }

    pub fn builder() -> SpecBuilder {
        SpecBuilder::new()
    }
}

pub struct SpecBuilder {
    status: Option<u16>,
    header: Option<Header>,
}

impl SpecBuilder {
    fn new() -> Self {
        SpecBuilder {
            status: None,
            header: None,
        }
    }

    pub fn status(mut self, status: u16) -> SpecBuilder {
        self.status = Some(status);
        self
    }

    pub fn header<T: Into<Header>>(mut self, header: T) -> SpecBuilder {
        self.header = Some(header.into());
        self
    }

    pub fn build(self) -> Spec {
        Spec {
            status: self.status,
            header: self.header,
        }
    }
}
