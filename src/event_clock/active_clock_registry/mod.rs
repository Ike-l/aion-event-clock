use std::{collections::HashSet, sync::Arc};

use aion_program::prelude::{Resource, ProgramRegistryResolveWithInsert, AccessBuilder, ResourceId, ProgramRegistry, ProgramRegistryReplaceResourceError, ResolveResourceError, AccessSubmissionError};
use aion_processor::prelude::{Unique};

use crate::prelude::{ActiveClock};

pub mod active_clock;

/// # ActiveClock Registry
/// 
/// Holds all `ActiveClock Event`s; inactive, and active
pub type ActiveClockRegistry = HashSet<ActiveClock>;

pub const ACTIVE_CLOCK_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventActiveClock EventRegistry");

pub const ACTIVE_CLOCK_REGISTRY_ACCESS_BUILDER: AccessBuilder<'static> = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(ACTIVE_CLOCK_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_mut_active_clock_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Unique<'a, ActiveClockRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Unique<ActiveClockRegistry>>(
        vec![ACTIVE_CLOCK_REGISTRY_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(ActiveClockRegistry::default()))), 
            resource_id: Some(ACTIVE_CLOCK_REGISTRY_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id is None
    ).unwrap()
}