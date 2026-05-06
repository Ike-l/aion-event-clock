use std::{collections::HashSet, sync::Arc};

use aion_program::prelude::{Resource, ProgramRegistryResolveWithInsert, AccessBuilder, ResourceId, ProgramRegistry, ProgramRegistryReplaceResourceError, ResolveResourceError, AccessSubmissionError};
use aion_processor::prelude::{Shared};

use crate::prelude::{Clock};

/// # Clock Registry
/// 
/// Holds all `Clock Event`s; inactive, and active
pub type ClockRegistry = HashSet<Clock>;

pub const CLOCK_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventClock EventRegistry");

pub const CLOCK_REGISTRY_ACCESS_BUILDER: AccessBuilder<'static> = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(CLOCK_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_clock_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Shared<'a, ClockRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Shared<ClockRegistry>>(
        vec![CLOCK_REGISTRY_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(ClockRegistry::default()))), 
            resource_id: Some(CLOCK_REGISTRY_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id is None
    ).unwrap()
}