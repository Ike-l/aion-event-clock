// at absolute times, when events
// Event History
// Can check with Event History

use std::{sync::Arc, time::Instant};

use aion_event::prelude::{EventBuffer, EventHistory, EventSystem};
use aion_program::prelude::ProgramRegistry;

use crate::prelude::{get_mut_current_clock};

pub mod clock;
pub mod current_clock;

// When (Tick - Start) % Interval == Stage 
// Only spawn an alert when the timer has elapsed (not at start like while)
pub struct EventClock;

impl EventSystem for EventClock {
    fn execute(
        program_registry: &Arc<ProgramRegistry>, 
        current_events: &EventBuffer,
        event_histry: &EventHistory,
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let current_clock = get_mut_current_clock(program_registry);
        if let Ok(Ok(Ok(mut current_clock))) = current_clock {
            current_clock.as_mut().update();
        }
        // Get ClockRegistry, ActiveClockRegistry
        // Find all new Active
        // Insert into ActiveClockRegistry (with Latest)
        
        // Get TimerRegistry
        // Find all in Window from ActiveClockRegistry
        // Remove Expired from TimerRegistry
        // Insert into TimerRegistry

        // For each Timer
        // If Elapsed then spawn Alert
        // 


        event_buffer
    }
}