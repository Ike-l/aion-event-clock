use std::{sync::Arc, time::Instant};

use aion_program::prelude::{Unique, ProgramRegistryResolveWithInsert, ProgramRegistryReplaceResourceError, ResolveResourceError, AccessSubmissionError, ResourceId, Resource, AccessBuilder, ProgramRegistry};
use crate::prelude::{Tick, ClockInstant, ClockDuration};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ClockCapture {
    ticks: Tick,
    time: Instant,
}

impl Default for ClockCapture {
    fn default() -> Self {
        Self { ticks: Tick::default(), time: Instant::now() }
    }
}

impl ClockCapture {
    pub fn ticks(&self) -> &Tick {
        &self.ticks
    }

    pub fn time(&self) -> &Instant {
        &self.time
    }

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
    }
}

pub const CURRENT_CLOCK_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventClock CurrentClock");

pub const CURRENT_CLOCK_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(CURRENT_CLOCK_RESOURCE_ID),
    resource_password: None
};

pub fn get_mut_current_clock<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Unique<'a, ClockCapture>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Unique<ClockCapture>>(
        None,
        vec![CURRENT_CLOCK_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(ClockCapture::default()))), 
            resource_id: Some(CURRENT_CLOCK_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id is None
    ).unwrap()
}