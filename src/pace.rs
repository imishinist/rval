use std::time::Duration;

#[derive(Debug, Eq, PartialEq)]
pub enum PaceState {
    Immediately,
    Wait(Duration),
}

pub trait Pacer {
    fn pace(&self, elapsed: Duration, hits: u128) -> PaceState;
}

mod rate;
pub use rate::*;

mod non_stop;
pub use non_stop::*;
