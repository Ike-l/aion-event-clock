use crate::prelude::{Tick};
use std::{sync::Arc, time::Instant};

use aion_processor::prelude::{Unique};
use aion_program::prelude::{ProgramRegistryResolveWithInsert, ProgramRegistryReplaceResourceError, ResolveResourceError, AccessSubmissionError, ResourceId, Resource, AccessBuilder, ProgramRegistry};

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