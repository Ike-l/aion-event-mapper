use std::{sync::Arc};

use aion_event::prelude::{EventSystem, EventBuffer};
use aion_program::prelude::{ProgramRegistry};
use crate::{event_mapper::or_registry::get_or_registry, prelude::get_and_registry};

pub mod and_registry;
pub mod or_registry;

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
pub struct EventMapper {}

impl EventSystem for EventMapper {
    fn execute(program_registry: &Arc<ProgramRegistry>, current_events: &EventBuffer) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        match get_and_registry(program_registry) {
            Ok(Ok(Ok(and_registry))) => {
                event_buffer.extend(current_events.read().filter_map(|current_event| {
                    and_registry.as_ref().get(current_event).cloned()
                }));
            },
            _ => (),
        }


        match get_or_registry(program_registry) {
            Ok(Ok(Ok(or_registry))) => {
                event_buffer.extend(
                    or_registry
                        .as_ref()
                        .iter()
                        .filter(|(target_event, _)| {
                            !current_events.contains(target_event)
                        })
                        .map(|(_, new_event)| new_event.clone())
                );
            },
            _ => ()
        }

        event_buffer
    }
}