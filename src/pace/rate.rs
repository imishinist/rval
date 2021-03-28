use std::ops::Sub;
use std::time::Duration;

use crate::pace::{PaceState, Pacer};

pub struct Rate {
    freq: u128,
    per: Duration,
}

impl Rate {
    pub fn new(freq: u128, per: Duration) -> Rate {
        assert!(freq > 0);
        assert!(per > Duration::from_secs(0));
        Rate { freq, per }
    }
}

impl Pacer for Rate {
    fn pace(&self, elapsed: Duration, hits: u128) -> PaceState {
        let n = elapsed.as_nanos() / self.per.as_nanos();
        let expected = self.freq * n;
        if hits < expected {
            return PaceState::Immediately;
        }

        let interval = self.per.as_nanos() / self.freq;
        let delta = Duration::from_nanos(((hits + 1) * interval) as u64);
        if delta < elapsed {
            return PaceState::Immediately;
        }

        PaceState::Wait(delta.sub(elapsed))
    }
}

#[cfg(test)]
mod tests {
    use crate::pace::*;
    use std::time::Duration;

    #[test]
    fn rate_test() {
        let sec = Duration::from_secs(1);

        let table = vec![
            ((1, sec), (1 * sec, 0), PaceState::Immediately),
            ((1, sec), (2 * sec, 0), PaceState::Immediately),
            ((1, sec), (1 * sec, 1), PaceState::Wait(sec)),
            ((1, sec), (1 * sec, 2), PaceState::Wait(2 * sec)),
            ((1, sec), (1 * sec, 10), PaceState::Wait(10 * sec)),
            ((1, sec), (11 * sec, 10), PaceState::Immediately),
            (
                (2, sec),
                (49 * sec / 10, 9),
                PaceState::Wait(Duration::from_millis(100)),
            ),
        ];

        for ((freq, per), (elapsed, hits), state) in table {
            let r = Rate::new(freq, per);
            let gstate = r.pace(elapsed, hits);
            assert_eq!(gstate, state);
        }
    }
}
