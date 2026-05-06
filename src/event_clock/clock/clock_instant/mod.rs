use crate::prelude::{Tick};
use std::time::Instant;

pub enum ClockInstant {
    Tick(Tick),
    Time(Instant)
}
