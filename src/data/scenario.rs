use crate::data::{Request, Spec};

#[derive(Debug)]
pub struct Scenario {
    name: String,
    url: String,

    num: usize,

    spec: Spec,
}

impl Scenario {
    pub fn new(name: String, url: String, num: usize, spec: Spec) -> Self {
        Scenario {
            name,
            url,
            num,
            spec,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn const_iter(&self) -> impl Iterator<Item = Request> + '_ {
        ConstantIter {
            scenario: &self,
            pos: 0,
            num: self.num,
        }
    }

    pub fn spec(&self) -> &Spec {
        &self.spec
    }
}

pub struct ConstantIter<'a> {
    scenario: &'a Scenario,
    pos: usize,
    num: usize,
}

impl<'a> Iterator for ConstantIter<'a> {
    type Item = Request;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.num {
            self.pos += 1;
            return Some(Request::new(self.scenario.url().clone()));
        }
        None
    }
}
