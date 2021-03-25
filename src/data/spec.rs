#[derive(Debug)]
pub struct Spec {
    pub status: Option<u16>,
}

pub struct SpecBuilder {
    status: Option<u16>,
}

impl SpecBuilder {
    pub fn new() -> Self {
        SpecBuilder { status: None }
    }

    pub fn status(mut self, status: u16) -> SpecBuilder {
        self.status = Some(status);
        self
    }

    pub fn build(self) -> Spec {
        Spec {
            status: self.status,
        }
    }
}
