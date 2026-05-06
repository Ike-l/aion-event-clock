use crate::prelude::{ClockDuration, ClockInstant};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ClockInterval {
    duration: ClockDuration, 
    // dont know how ill use interval moment as Time yet can put unimplemented!()
    interval_moment: ClockInstant,
}