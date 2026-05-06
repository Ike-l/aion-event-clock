use crate::prelude::{ClockInstant, Tick, ClockDuration};

pub enum ClockFinish {
    ClockInstant(ClockInstant),
    IntervalLimit(Tick),
    TTL(ClockDuration)
}