use anyhow::Result;

use rval::data::SpecBuilder;
use rval::{data::Scenario, play::Player};

fn main() -> Result<()> {
    let spec = SpecBuilder::new().status(200).build();
    let scenario = Scenario::new("test".into(), "https://google.com".into(), 5, spec);
    let player = Player::new(scenario);
    player.play()?;

    Ok(())
}
