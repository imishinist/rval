use std::time::Duration;

use anyhow::Result;
use reqwest::blocking;

use crate::data::{Response, Scenario};
use crate::validation::validate;

#[derive(Debug)]
pub struct Player {
    scenario: Scenario,
}

impl Player {
    pub fn new(scenario: Scenario) -> Self {
        Player { scenario }
    }

    pub fn play(&self) -> Result<()> {
        let s = &self.scenario;
        let client = reqwest::blocking::Client::builder()
            .pool_idle_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(10)
            .build()?;
        let spec = s.spec();
        for req in s.const_iter() {
            let res = Response::from(client.get(req.url()).send()?);
            match validate(spec, res) {
                Ok(_) => {
                    println!("[{}]: {} => OK", s.name(), s.url());
                }
                Err(e) => {
                    eprintln!("{}", e.to_string());
                }
            }
        }
        Ok(())
    }
}
