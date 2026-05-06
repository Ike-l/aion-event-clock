use crate::prelude::{Tick, ClockDuration};
use std::time::Instant;

pub enum ClockInstant {
    Tick(Tick),
    Time(Instant)
}

impl ClockInstant {
    pub fn checked_add(&self, clock_duration: &ClockDuration) -> Option<Option<Self>> {
        match (self, clock_duration) {
            (Self::Tick(tick_instant), ClockDuration::Tick(tick_duration)) => {
                Some(tick_instant.checked_add(tick_duration).map(|tick| Self::Tick(tick)))
            },
            (Self::Time(instant), ClockDuration::Duration(duration)) => {
                Some(instant.checked_add(*duration).map(|instant| Self::Time(instant)))
            }
            _ => None
        }
    }
}
