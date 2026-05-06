use std::{collections::HashMap, sync::Arc};

use aion_event::prelude::{Event};
use aion_program::prelude::{ResourceId, ResourceAccess, Resource, ProgramReplaceResource, ProgramRegistry, AccessBuilder, ResolveResourceError};
use aion_processor::prelude::{Shared};
use aion_state::prelude::{RegistrySaferReplacementResult};

/// # AND Registry
/// Each entry is composed of 2 `Event`s:
/// * L: Checking Event
/// * R: New Event
/// 
/// If `Checking Event` is in `current_events` then it will spawn `New Event`
pub type AndRegistry = HashMap<Event, Event>;

pub const AND_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventMapper AndRegistry");

pub const AND_REGISTRY_ACCESS_BUILDER: AccessBuilder<'static> = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(AND_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_and_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Shared<'a, AndRegistry> {
    match program_registry
        .resolve::<Shared<AndRegistry>>(
            vec![AND_REGISTRY_ACCESS_BUILDER]
        ) {
            Ok(result) => {
                match result {
                    Ok(and_registry) => and_registry,
                    Err(resolve_error) => {
                        match resolve_error {
                            ResolveResourceError::Resolving => {
                                let replace_result = program_registry.replace_resource(ProgramReplaceResource { 
                                    user_details: None,
                                    program_id: None,
                                    program_password: None, 
                                    resource: Some(Resource::new::<AndRegistry>(HashMap::new())), 
                                    access: &ResourceAccess::Replace, 
                                    resource_id: AND_REGISTRY_RESOURCE_ID, 
                                    resource_password: None 
                                });

                                assert!(matches!(replace_result, Ok(RegistrySaferReplacementResult::Found(_))));

                                let Ok(Ok(and_registry)) = program_registry.resolve::<Shared<AndRegistry>>(vec![AND_REGISTRY_ACCESS_BUILDER]) else { unreachable!() };

                                and_registry
                            },
                            ResolveResourceError::Casting |
                            ResolveResourceError::NotEnoughResults |
                            ResolveResourceError::TooManyResults => unreachable!(),
                        }
                    }
                }
            },
            Err(_) => unreachable!(),
        }
}