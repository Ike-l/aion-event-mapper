use std::{sync::Arc};

use aion_ecs::prelude::Query;
use aion_event::prelude::{EventBuffer, EventHistory, EventSystem};
use aion_program::prelude::{ProgramRegistry, Shared};
use tokio::runtime::Runtime;

use crate::prelude::{AndMapping, GetAndMappings, OrMapping};

pub mod and_mapping;
pub mod or_mapping;

pub mod get_and_mappings;
pub mod get_or_mappings;

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
        program_registry: &Arc<ProgramRegistry>, 
        current_events: &EventBuffer,
        _event_history: &EventHistory,
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let runtime = program_registry.resolve::<Shared<Runtime>>(None, vec![]);
        let runtime = match runtime {
            Ok(runtime) => Some(runtime),
            _ => None
        };
        /*
        make trait functions to get the AndMapping's with the runtime
        Same for OrMapping

        */
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

        {
            let or_mappings = program_registry.resolve::<Query<&OrMapping>>(None, vec![]);
            if let Ok(Ok(or_mappings)) = or_mappings {
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