// at absolute times, when events
// Event History
// Can check with Event History

use std::{collections::HashSet, sync::Arc};

use aion_event::prelude::{EventBuffer, EventHistory, EventSystem};
use aion_program::prelude::ProgramRegistry;

use crate::prelude::{Timer, get_mut_current_clock, get_clock_registry, get_mut_timer_registry};

pub mod clock;
pub mod clock_capture;
pub mod timer_registry;
pub mod clock_registry;

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

        let current_clock = current_clock.as_ref();

        let triggered_clocks = match get_clock_registry(program_registry) {
            Ok(Ok(Ok(clock_registry))) => {
                Some(clock_registry.as_ref().iter().filter(|clock| clock.triggered(current_events)).cloned().collect::<HashSet<_>>())
            },
            _ => None
        };

        match get_mut_timer_registry(program_registry) {
            Ok(Ok(Ok(mut timer_registry))) => {
                let timer_registry = timer_registry.as_mut();

                let mut continuing_clocks = HashSet::new();

                for mut timer in timer_registry.drain() {
                    if timer.alive(current_clock) {
                        if timer.elapsed(current_clock) {
                            if let Some(alert) = timer.alert() {
                                event_buffer.insert(alert.clone());
                            }
                        }
    
                        continuing_clocks.insert(timer);
                    } else {
                        if let Some(final_alert) = timer.final_alert() {
                            event_buffer.insert(final_alert.clone());
                        }
                    }
                }

                // Adds later because we know if they have just been added then they can't have elapsed
                if let Some(triggered_clocks) = triggered_clocks {
                    timer_registry.extend(triggered_clocks.into_iter().map(|triggered_clock| Timer::new(triggered_clock, current_clock.clone())));
                }

                timer_registry.extend(continuing_clocks);
            },
            _ => ()
        };

        event_buffer
    }
}