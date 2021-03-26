use anyhow::Result;

use rval::data::Spec;
use rval::{data::Scenario, play::Player};

fn main() -> Result<()> {
    let spec = Spec::builder().status(200).build();
    let scenario = Scenario::new("test".into(), "https://google.com".into(), 5, spec);
    let player = Player::new(scenario);
    player.play()?;

    Ok(())
}
