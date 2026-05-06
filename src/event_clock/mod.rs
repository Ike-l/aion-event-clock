// at absolute times, when events
// Event History
// Can check with Event History

use std::{collections::HashSet, sync::Arc};

use aion_event::prelude::{EventBuffer, EventHistory, EventSystem};
use aion_program::prelude::ProgramRegistry;

use crate::prelude::{get_mut_current_clock, get_clock_registry, get_mut_active_clock_registry};

pub mod clock;
pub mod clock_capture;
pub mod active_clock_registry;
pub mod clock_registry;

// When (Tick - Start) % Interval == Stage 
// Only spawn an alert when the timer has elapsed (not at start like while)
pub struct EventClock;

impl EventSystem for EventClock {
    fn execute(
        program_registry: &Arc<ProgramRegistry>, 
        current_events: &EventBuffer,
        _event_history: &EventHistory,
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let current_clock = get_mut_current_clock(program_registry);

        let Ok(Ok(Ok(mut current_clock))) = current_clock else { return event_buffer };
        current_clock.as_mut().update();

        let new_active_clocks = match get_clock_registry(program_registry) {
            Ok(Ok(Ok(clock_registry))) => {
                Some(clock_registry.as_ref().iter().filter(|clock| clock.triggered(current_events)).cloned().collect::<HashSet<_>>())
            },
            _ => None
        };

        match get_mut_active_clock_registry(program_registry) {
            Ok(Ok(Ok(active_clock_registry))) => {
                // insert new active clocks
                // with "birth" being current_clock cloned and interval_count as 0
                // // return all active clocks with Alive
                // only retain those that are alive
                // and for each
                // if elapsed then spawn alert

            },
            _ => ()
        };

        event_buffer
    }
}