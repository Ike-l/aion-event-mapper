use std::{collections::HashMap, sync::Arc};

use aion_event::prelude::{Event};
use aion_program::prelude::{ProgramRegistryReplaceResourceError, AccessSubmissionError, ProgramRegistryResolveWithInsert, ResourceId, Resource, ProgramRegistry, AccessBuilder, ResolveResourceError};
use aion_processor::prelude::{Shared};

/// # OR Registry
/// Each entry is composed of 2 `Event`s:
/// * L: Checking Event
/// * R: New Event
/// 
/// If `Checking Event` is not in `current_events` then it will spawn `New Event`
pub type OrRegistry = HashMap<Event, Event>;

pub const OR_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventMapper OrRegistry");

pub const OR_REGISTRY_ACCESS_BUILDER: AccessBuilder<'static> = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(OR_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_or_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Shared<'a, OrRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Shared<OrRegistry>>(
        vec![OR_REGISTRY_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Resource::new(OrRegistry::default())), 
            resource_id: Some(OR_REGISTRY_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id is None
    ).unwrap()
}