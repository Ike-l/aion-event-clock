use std::time::Instant;

use crate::prelude::{Tick, Clock, ClockCapture};

use aion_event::prelude::Event;

#[derive(PartialEq, Eq, Hash)]
pub struct ActiveClock {
    clock: Clock,
    interval_count: Tick,
    birth: ClockCapture,
    last_checked_time: Instant,
}

impl ActiveClock {
    pub fn new(clock: Clock, birth: ClockCapture) -> Self {
        Self { 
            clock, 
            interval_count: Tick::default(), 
            last_checked_time: birth.time().clone(),
            birth, 
        }
    }

    pub fn alive(&self, current_clock: &ClockCapture) -> bool {
        self.clock.alive(current_clock, &self.interval_count, &self.birth)
    }

    pub fn alert(&self) -> &Option<Event> {
        &self.clock.alert
    }

    pub fn final_alert(&self) -> &Option<Event> {
        &self.clock.final_alert
    }

    pub fn elapsed(&mut self, current_clock: &ClockCapture) -> bool {
        let result = self.clock.elapsed(current_clock, &self.birth, &self.last_checked_time);

        self.last_checked_time = current_clock.time().clone();

        result
    }
}