use aion_event::prelude::Event;

use crate::prelude::{ClockInstant, ClockFinish, ClockInterval};

pub mod tick;
pub mod clock_instant;
pub mod clock_interval;
pub mod clock_duration;
pub mod clock_finish;

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

    // If None doesnt replaceBut
    pub setup_next_interval: Option<Box<dyn FnMut(Option<ClockInterval>) -> Option<Option<ClockInterval>> >>
}
