use anyhow::{anyhow, Context, Result};

use rval::data::{Request, Scenario};

fn run(scenario: Scenario) -> Result<()> {
    for request in scenario.const_iter() {
        let resp = reqwest::blocking::get(request.url())?;
        println!("{:#?}", resp);
    }
    Ok(())
}

fn main() -> Result<()> {
    let scenario = Scenario::new("test".into(), "https://google.com".into(), 10);
    run(scenario)?;

    Ok(())
}
