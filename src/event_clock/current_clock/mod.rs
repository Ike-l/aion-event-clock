use std::{sync::Arc, time::Instant};

use aion_processor::prelude::{Unique};
use aion_program::prelude::{ProgramRegistryResolveWithInsert, ProgramRegistryReplaceResourceError, ResolveResourceError, AccessSubmissionError, ResourceId, Resource, AccessBuilder, ProgramRegistry};
use crate::prelude::{Tick, ClockInstant, ClockDuration};

pub struct CurrentClock {
    ticks: Tick,
    time: Instant,
}

impl Default for CurrentClock {
    fn default() -> Self {
        Self { ticks: Tick::default(), time: Instant::now() }
    }
}

impl CurrentClock {
    pub fn update(&mut self) -> Option<(Tick, Instant)> {
        let old_tick = self.ticks.increment()?;
        let old_time = self.time;

        self.time = Instant::now();

        Some((old_tick, old_time))
    }

    pub fn is_before(&self, clock_instant: &ClockInstant) -> bool {
        match clock_instant {
            ClockInstant::Tick(tick) => self.ticks < *tick,
            ClockInstant::Time(instant) => self.time.checked_duration_since(*instant).is_none(),
        }
    }

    pub fn is_after(&self, clock_instant: &ClockInstant) -> bool {
        match clock_instant {
            ClockInstant::Tick(tick) => self.ticks > *tick ,
            ClockInstant::Time(instant) => self.time.checked_duration_since(*instant).is_some_and(|duration| !duration.is_zero()),
        }
    }

    pub fn since_is_greater(&self, start: &Self, duration: &ClockDuration) -> bool {
        match duration {
            ClockDuration::Duration(duration) => {
                let lifetime = self.time.checked_duration_since(start.time);
                if let Some(lifetime) = lifetime {
                    lifetime > *duration
                } else {
                    false
                }
            },
            ClockDuration::Tick(tick) => {
                let expected_finish = start.ticks.checked_add(tick);
                if let Some(expected_finish) = expected_finish {
                    self.ticks > expected_finish
                } else {
                    false
                }
            },
        }

        // duration since start is greater than duration

        // birth + clock_duration < current_clock
        // match finish_time {
        //     Some(Some(finish_time)) => current_clock.is_after(&finish_time),
        //     // If unable to compute the expected finish time say its not finished
        //     // - since somewhere else will catch it when it does actually reach the maximum value
        //     Some(None) => false,
        // }
        // if start + duration is after self
    }
}

pub const CURRENT_CLOCK_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventClock CurrentClock");

pub const CURRENT_CLOCK_ACCESS_BUILDER: AccessBuilder<'static> = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(CURRENT_CLOCK_RESOURCE_ID),
    resource_password: None
};

pub fn get_mut_current_clock<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Unique<'a, CurrentClock>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Unique<CurrentClock>>(
        vec![CURRENT_CLOCK_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(CurrentClock::default()))), 
            resource_id: Some(CURRENT_CLOCK_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id is None
    ).unwrap()
}