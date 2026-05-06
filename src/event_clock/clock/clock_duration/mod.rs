use crate::prelude::{Tick};
use std::time::Duration;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ClockDuration {
    Duration(Duration),
    Tick(Tick)
}