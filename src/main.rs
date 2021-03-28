use std::time::Duration;

use anyhow::{anyhow, Result};
use clap::{clap_app, crate_authors, crate_description, crate_version};

use rval::data::Spec;
use rval::pace::{NonStop, Pacer, Rate};
use rval::{data::Scenario, play::Player};

fn main() -> Result<()> {
    let matches = clap_app!(rval =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg name: --name [NAME] "name of scenario")
        (@arg status: -s --status [STATUS] "http status code")
        (@arg url: --url [URL] "request url")
        (@arg num: -n --num [NUM] "request count")
        (@arg worker: -w --worker [NUM] "workers count")
        (@arg freq: -f --freq [NUM] "request frequency")
        (@arg pacer: -p --pacer [PACER] possible_values(&["rate", "non-stop"]) default_value("rate") "pace algorithm")
    )
    .get_matches();

    let name = matches.value_of("name").unwrap_or("ping");
    let status = matches.value_of("status").unwrap_or("200").parse::<u16>()?;
    let url = matches.value_of("url").unwrap();
    let num = matches.value_of("num").unwrap_or("5").parse::<usize>()?;
    let worker = matches.value_of("worker").unwrap_or("5").parse::<usize>()?;
    let freq = matches.value_of("freq").unwrap_or("1").parse::<u128>()?;

    let pacer = match matches.value_of("pacer").unwrap() {
        "rate" => Ok(Box::new(Rate::new(freq, Duration::from_secs(1))) as Box<dyn Pacer>),
        "non-stop" => Ok(Box::new(NonStop::new()) as Box<dyn Pacer>),
        s => Err(anyhow!("unrecognized pace algorithm: {}", s)),
    }?;

    let player = Player::new(worker);

    let spec = Spec::builder().status(status).build();
    let scenario = Scenario::new(name.into(), url.into(), num, spec);
    player.play(pacer, scenario)?;

    Ok(())
}
