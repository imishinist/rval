use std::time::Duration;

use anyhow::{anyhow, Result};
use clap::{clap_app, crate_authors, crate_description, crate_version, value_t};

use rval::data::Spec;
use rval::pace::{NonStop, Pacer, Rate};
use rval::{data::Scenario, play::Player};

fn main() -> Result<()> {
    env_logger::init();
    let matches = clap_app!(rval =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg name: --name [NAME] "name of scenario")
        (@arg url: --url [URL] "request url")
        (@arg status: -s --status [STATUS] "http status code")
        (@arg num: -n --num [NUM] "request count")
        (@arg worker: -w --worker [NUM] "workers count")
        (@arg freq: -f --freq [NUM] "request frequency")
        (@arg pacer: -p --pacer [PACER] possible_values(&["rate", "non-stop"]) default_value("rate") "pace algorithm")
    )
    .get_matches();

    let name = matches.value_of("name").unwrap_or("ping");
    let url = matches.value_of("url").expect("specify url");

    // for spec
    let status = value_t!(matches, "status", u16).unwrap_or(200);

    // for run setting
    let num = value_t!(matches, "num", u64).unwrap_or(5);
    let worker = value_t!(matches, "worker", usize).unwrap_or(5);
    let freq = value_t!(matches, "freq", u128).unwrap_or(1u128);

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
