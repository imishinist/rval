use std::time::Duration;

use crate::pace::{PaceState, Pacer};

pub struct NonStop;

impl NonStop {
    pub fn new() -> Self {
        NonStop
    }
}

impl Pacer for NonStop {
    fn pace(&self, _elapsed: Duration, _hits: u128) -> PaceState {
        PaceState::Immediately
    }
}
