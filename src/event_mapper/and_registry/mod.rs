use std::{collections::HashMap, sync::Arc};

use aion_event::prelude::{Event};
use aion_program::prelude::{ResourceId, ProgramRegistryResolveWithInsert, ProgramRegistryReplaceResourceError, AccessSubmissionError, Resource, ProgramRegistry, AccessBuilder, ResolveResourceError};
use aion_processor::prelude::{Shared};

/// # AND Registry
/// Each entry is composed of 2 `Event`s:
/// * L: Checking Event
/// * R: New Event
/// 
/// If `Checking Event` is in `current_events` then it will spawn `New Event`
pub type AndRegistry = HashMap<Event, Event>;

pub const AND_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventMapper AndRegistry");

pub const AND_REGISTRY_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(AND_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_and_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Shared<'a, AndRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Shared<AndRegistry>>(
        vec![AND_REGISTRY_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(AndRegistry::default()))), 
            resource_id: Some(AND_REGISTRY_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id or resource is None
    ).unwrap()
}