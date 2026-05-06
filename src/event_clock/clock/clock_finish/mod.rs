use crate::prelude::{ClockInstant, Tick, ClockDuration};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ClockFinish {
    ClockInstant(ClockInstant),
    IntervalLimit(Tick),
    TTL(ClockDuration)
}