use std::time::Instant;

use crate::prelude::{ClockDuration, ClockInstant, ClockCapture, };

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ClockInterval {
    duration: ClockDuration, 
    // dont know how ill use interval moment as Time yet can put unimplemented!()
    interval_moment: ClockInstant,
}

impl ClockInterval {
    pub fn elapsed(&self, start: &Option<ClockInstant>, current_clock: &ClockCapture, birth: &ClockCapture, last_checked_time: &Instant) -> bool {
        match (&self.duration, &self.interval_moment) {
            (ClockDuration::Duration(duration), ClockInstant::Tick(_)) => {
                current_clock.time().duration_since(*last_checked_time) > *duration
            },
            (ClockDuration::Tick(tick_duration), ClockInstant::Tick(interval_moment)) => {
                if current_clock.ticks() == birth.ticks() {
                    return false;
                }
                
                let ticks_since_the_beginning = if let Some(_) = start {
                    let Some(ticks_since_the_beginning) = current_clock.ticks().checked_sub(birth.ticks()) else { return false };
                    ticks_since_the_beginning
                } else {
                    current_clock.ticks().clone()
                };

                let rem = ticks_since_the_beginning.rem_euclid(tick_duration);
                
                match rem {
                    Some(rem) => rem == *interval_moment,
                    None => true,
                }
            },
            (ClockDuration::Duration(_duration), ClockInstant::Time(_instant)) => unimplemented!(),
            (ClockDuration::Tick(_tick), ClockInstant::Time(_instant)) => unimplemented!(),
        }
    }
}