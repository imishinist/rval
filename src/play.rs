use std::time::Duration;

use anyhow::Result;
use reqwest::blocking;

use crate::data::{Response, Scenario};
use crate::validation::validate;

#[derive(Debug)]
pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Player {}
    }

    pub fn play(&self, scenario: Scenario) -> Result<()> {
        let client = blocking::Client::builder()
            .pool_idle_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(10)
            .build()?;
        let spec = scenario.spec();
        for req in scenario.const_iter() {
            let res = Response::from(client.get(req.url()).send()?);
            match validate(spec, res) {
                Ok(_) => {
                    println!("[{}]: {} => OK", scenario.name(), scenario.url());
                }
                Err(e) => {
                    eprintln!("{}", e.to_string());
                }
            }
        }
        Ok(())
    }
}
