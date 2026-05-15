// at absolute times, when events
// Event History
// Can check with Event History

use std::{collections::{HashMap, HashSet}, sync::Arc};

use aion_ecs::prelude::{Query, UNIQUE_WORLD_ACCESS_BUILDER, WORLD_RESOURCE_ID, World};
use aion_event::prelude::{EventBuffer, EventHistory, EventSystem};
use aion_program::prelude::{ProgramRegistry, ProgramRegistryResolveWithInsert, Resource, Unique};
use hecs::Entity;

use crate::prelude::{Clock, Timer, get_mut_current_clock};

pub mod clock;
pub mod clock_capture;
pub mod timer;

pub struct EventClock;

impl EventSystem for EventClock {
    fn execute(
        &self,
        program_registry: &Arc<ProgramRegistry>, 
        current_events: &EventBuffer,
        _event_history: &EventHistory,
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let current_clock = get_mut_current_clock(program_registry);

        let Ok(Ok(Ok(mut current_clock))) = current_clock else { return event_buffer };
        current_clock.as_mut().update();

        let current_clock = current_clock.as_ref();

        let mut dead_timers = HashSet::new();
        {
            let timers = program_registry.resolve::<Query<(Entity, &mut Timer)>>(vec![]);
            if let Ok(Ok(mut timers)) = timers {
                for (entity, timer) in timers.borrow().iter() {
                    if timer.alive(current_clock) {
                        if timer.elapsed(current_clock) {
                            if let Some(alert) = timer.alert() {
                                event_buffer.insert(alert.clone());
                            }
                        }
                    } else {
                        if let Some(final_alert) = timer.final_alert() {
                            event_buffer.insert(final_alert.clone());
                        }

                        dead_timers.insert(entity);
                    }
                }
            }
        }

        {
            let world = program_registry.resolve_with_insert::<Unique<World>>(vec![UNIQUE_WORLD_ACCESS_BUILDER], ProgramRegistryResolveWithInsert {
                resource: Some(Box::new(|| Resource::new(World::default()))),
                resource_id: Some(WORLD_RESOURCE_ID),
                ..Default::default()
            }).expect("Resource and ResourceId are Some");
    
            {
                if let Ok(Ok(Ok(mut world))) = world {
                    for dead_timer in dead_timers {                    
                        let _ = world.as_mut().remove::<(Timer,)>(dead_timer);
                    }
                }
            }
        }

        let mut triggered_clocks = HashMap::new();
        {    
            let clocks = program_registry.resolve::<Query<(Entity, &Clock)>>(vec![]);
            if let Ok(Ok(mut clocks)) = clocks {
                for (entity, clock) in clocks.borrow().iter() {
                    if clock.triggered(current_events) {
                        triggered_clocks.insert(entity, Timer::new(clock.clone(), current_clock.clone()));
                    }
                }
            }
        }

        {
            let world = program_registry.resolve_with_insert::<Unique<World>>(vec![UNIQUE_WORLD_ACCESS_BUILDER], ProgramRegistryResolveWithInsert {
                resource: Some(Box::new(|| Resource::new(World::default()))),
                resource_id: Some(WORLD_RESOURCE_ID),
                ..Default::default()
            }).expect("Resource and ResourceId are Some");
            
            if let Ok(Ok(Ok(mut world))) = world {
                for (entity, timer) in triggered_clocks {
                    let _ = world.as_mut().insert(entity, (timer,));
                }
            }
        }

        event_buffer
    }
}