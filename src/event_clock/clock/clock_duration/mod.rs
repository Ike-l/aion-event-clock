use crate::prelude::{Tick};
use std::time::Duration;

pub enum ClockDuration {
    Duration(Duration),
    Tick(Tick)
}