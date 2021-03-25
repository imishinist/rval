use anyhow::{anyhow, Context, Result};

use rval::data::{Request, Scenario};

fn main() -> Result<()> {
    let scenario = Scenario::new("test".into(), "https://google.com".into());
    let request = Request::new(scenario.url().clone());
    let resp = reqwest::blocking::get(request.url())?;
    println!("{:#?}", resp);

    Ok(())
}
