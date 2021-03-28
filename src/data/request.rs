use crate::data::Scenario;

pub struct Request {
    seq: u64,
    scenario_name: String,
    url: String,
}

impl Request {
    pub fn new(scenario_name: String, url: String) -> Self {
        Request {
            seq: 0,
            scenario_name,
            url,
        }
    }

    pub fn set_seq(&mut self, seq: u64) {
        self.seq = seq;
    }

    pub fn seq(&self) -> u64 {
        self.seq
    }

    pub fn scenario_name(&self) -> &String {
        &self.scenario_name
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

impl From<&Scenario> for Request {
    fn from(s: &Scenario) -> Self {
        Request::new(s.name().clone(), s.url().clone())
    }
}
