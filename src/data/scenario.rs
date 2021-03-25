pub struct Scenario {
    name: String,
    url: String,
}

impl Scenario {
    pub fn new(name: String, url: String) -> Self {
        Scenario { name, url }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}
