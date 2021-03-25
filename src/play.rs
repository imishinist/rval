use anyhow::Result;
use reqwest::blocking;

use crate::data::{Response, Scenario};

#[derive(Debug)]
pub struct Player {
    scenario: Scenario,
}

impl Player {
    pub fn new(scenario: Scenario) -> Self {
        Player { scenario }
    }

    pub fn play(&self) -> Result<()> {
        for req in self.scenario.const_iter() {
            let res = Response::from(blocking::get(req.url())?);

            println!("{:#?}", res);
        }
        Ok(())
    }
}
