use std::{collections::HashMap, sync::Arc};

use aion_event::prelude::{Event};
use aion_program::prelude::{ResourceId, ResourceAccess, Resource, ProgramReplaceResource, ProgramRegistry, AccessBuilder, ResolveResourceError};
use aion_processor::prelude::{Shared};
use aion_state::prelude::{RegistrySaferReplacementResult};

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
) -> Shared<'a, OrRegistry> {
    let or_registry_resource_id = ResourceId::Label("EventMapper OrRegistry".to_owned());

    match program_registry
        .resolve::<Shared<OrRegistry>>(
            vec![OR_REGISTRY_ACCESS_BUILDER]
        ) {
            Ok(result) => {
                match result {
                    Ok(or_registry) => or_registry,
                    Err(resolve_error) => {
                        match resolve_error {
                            ResolveResourceError::Resolving => {
                                let replace_result = program_registry.replace_resource(ProgramReplaceResource { 
                                    user_details: None,
                                    program_id: None,
                                    program_password: None, 
                                    resource: Some(Resource::new::<OrRegistry>(HashMap::new())), 
                                    access: &ResourceAccess::Replace, 
                                    resource_id: or_registry_resource_id.clone(), 
                                    resource_password: None 
                                });

                                assert!(matches!(replace_result, Ok(RegistrySaferReplacementResult::Found(_))));

                                let Ok(Ok(or_registry)) = program_registry
                                    .resolve::<Shared<OrRegistry>>(
                                        vec![OR_REGISTRY_ACCESS_BUILDER]
                                    ) else { unreachable!() };

                                or_registry
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