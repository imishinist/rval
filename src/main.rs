use anyhow::Result;

use rval::{data::Scenario, play::Player};

fn main() -> Result<()> {
    let scenario = Scenario::new("test".into(), "https://google.com".into(), 5);
    let player = Player::new(scenario);
    player.play()?;

    Ok(())
}
