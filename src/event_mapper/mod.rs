use std::{sync::Arc};

use aion_event::prelude::{EventBuffer, EventHistory, EventSystem};
use aion_program::prelude::ProgramRegistry;


#[cfg(any(feature = "and-mapping", feature = "or-mapping"))]
use tokio::runtime::Runtime;
#[cfg(any(feature = "and-mapping", feature = "or-mapping"))]
use aion_program::prelude::Shared;


#[cfg(feature = "and-mapping")]
pub mod and_mapping;
#[cfg(feature = "and-mapping")]
pub mod get_and_mappings;
#[cfg(feature = "and-mapping")]
use crate::prelude::GetAndMappings;

#[cfg(feature = "or-mapping")]
pub mod or_mapping;
#[cfg(feature = "or-mapping")]
pub mod get_or_mappings;
#[cfg(feature = "or-mapping")]
use crate::prelude::GetOrMappings;

/// # Event Mapper
/// Execute fetches the two registries from the `program_registry`
/// 
/// It then applies each respective semantics and returns the new events
/// 
/// The `OR Registry` and `AND Registry` are semantically opposite 
/// 
/// ## OR Registry
/// Each entry is composed of 2 `Event`s:
/// * L: Checking Event
/// * R: New Event
/// 
/// If `Checking Event` is not in `current_events` then it will spawn `New Event`
/// ## AND Registry
/// Each entry is composed of 2 `Event`s:
/// * L: Checking Event
/// * R: New Event
/// 
/// If `Checking Event` is in `current_events` then it will spawn `New Event`
pub struct EventMapper;

impl EventSystem for EventMapper {
    fn execute(
        &self,
        #[allow(unused_variables)] 
        program_registry: &Arc<ProgramRegistry>,
        #[allow(unused_variables)] 
        current_events: &EventBuffer,
        _event_history: &EventHistory,
    ) -> EventBuffer {
        #[allow(unused_mut)]
        let mut event_buffer = EventBuffer::default();

        #[cfg(any(feature = "and-mapping", feature = "or-mapping"))]
        let runtime = program_registry.resolve::<Shared<Runtime>>(None, vec![]);
        #[cfg(any(feature = "and-mapping", feature = "or-mapping"))]
        let runtime = match runtime {
            Ok(runtime) => Some(runtime),
            _ => None
        };
        
        #[cfg(feature = "and-mapping")]
        {
            let and_mappings = program_registry.get_and_mappings(runtime.as_deref());
            if let Ok(and_mappings) = and_mappings {
                for and_mapping in and_mappings.query().iter() {
                    if and_mapping.is_satisfied(current_events) {
                        event_buffer.insert(and_mapping.spawns().clone())
                    }
                }
            }
        }

        #[cfg(feature = "or-mapping")]
        {
            let or_mappings = program_registry.get_or_mappings(runtime.as_deref());
            if let Ok(or_mappings) = or_mappings {
                for or_mapping in or_mappings.query().iter() {
                    if or_mapping.is_satisfied(current_events) {
                        event_buffer.insert(or_mapping.spawns().clone())
                    }
                }
            }
        }

        event_buffer
    }
}