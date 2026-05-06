use std::time::Instant;

use aion_event::prelude::{Event, EventBuffer};

use crate::prelude::{ClockFinish, ClockInstant, ClockInterval, ClockCapture, Tick};

pub mod tick;
pub mod clock_instant;
pub mod clock_interval;
pub mod clock_duration;
pub mod clock_finish;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Clock {
    // If None then is always active
    pub condition: Option<Event>,
    
    // If None start when condition is met
    pub start: Option<ClockInstant>,
    
    // If None never finishes
    pub finish: Option<ClockFinish>,

    // If None Doesn't have an interval
    // - will spawn alert every tick
    pub interval: Option<ClockInterval>,

    pub alert: Option<Event>,
    pub final_alert: Option<Event>,

    // If None doesnt replaceBut
    // pub setup_next_interval: Option<Box<dyn FnMut(Option<ClockInterval>) -> Option<Option<ClockInterval>> >>
}

impl Clock {
    pub fn triggered(&self, current_events: &EventBuffer) -> bool {
        match &self.condition {
            Some(condition) => current_events.contains(condition),
            None => true,
        }
    }

    pub fn started(&self, current_clock: &ClockCapture) -> bool {
        match &self.start {
            Some(start) => current_clock.is_after(start),
            None => true,
        }
    }

    pub fn finished(&self, current_clock: &ClockCapture, interval_count: &Tick, birth: &ClockCapture) -> bool {
        match &self.finish {
            Some(finish) => {
                match finish {
                    ClockFinish::ClockInstant(clock_instant) => current_clock.is_before(clock_instant),
                    ClockFinish::IntervalLimit(limit) => interval_count > limit,
                    ClockFinish::TTL(clock_duration) => current_clock.since_is_greater(birth, clock_duration),
                }
            },
            None => false,
        }
    }

    pub fn alive(&self, current_clock: &ClockCapture, interval_count: &Tick, birth: &ClockCapture) -> bool {
        self.started(current_clock) && !self.finished(current_clock, interval_count, birth)        
    }

    pub fn elapsed(&self, current_clock: &ClockCapture, birth: &ClockCapture, last_checked_time: &Instant) -> bool {
        match &self.interval {
            Some(clock_interval) => clock_interval.elapsed(&self.start, current_clock, birth, last_checked_time),
            None => true,
        }
    }
}