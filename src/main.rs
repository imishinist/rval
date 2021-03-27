use anyhow::Result;

use clap::{clap_app, crate_authors, crate_description, crate_version};
use rval::data::Spec;
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
    )
    .get_matches();

    let name = matches.value_of("name").unwrap_or("ping");
    let status = matches.value_of("status").unwrap_or("200").parse::<u16>()?;
    let url = matches.value_of("url").unwrap();
    let num = matches.value_of("num").unwrap_or("5").parse::<usize>()?;

    let player = Player::new();

    let spec = Spec::builder().status(status).build();
    let scenario = Scenario::new(name.into(), url.into(), num, spec);
    player.play(scenario)?;

    Ok(())
}
