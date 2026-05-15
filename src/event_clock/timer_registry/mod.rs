use std::{collections::HashSet, sync::Arc};

use aion_program::prelude::{Unique, Resource, ProgramRegistryResolveWithInsert, AccessBuilder, ResourceId, ProgramRegistry, ProgramRegistryReplaceResourceError, ResolveResourceError, AccessSubmissionError};

use crate::prelude::{Timer};

pub mod timer;

/// # Timer Registry
/// 
/// Holds all `Timer Event`s; inactive, and active
pub type TimerRegistry = HashSet<Timer>;

pub const TIMER_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventTimer EventRegistry");

pub const TIMER_REGISTRY_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(TIMER_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_mut_timer_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Unique<'a, TimerRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Unique<TimerRegistry>>(
        vec![TIMER_REGISTRY_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(TimerRegistry::default()))), 
            resource_id: Some(TIMER_REGISTRY_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id or resource is None
    ).unwrap()
}