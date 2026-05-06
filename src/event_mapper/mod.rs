use std::{sync::Arc};

use aion_event::prelude::{EventSystem, EventBuffer};
use aion_program::prelude::{ProgramRegistry};
use crate::{event_mapper::or_registry::get_or_registry, prelude::get_and_registry};

pub mod and_registry;
pub mod or_registry;

pub struct EventMapper {}

impl EventSystem for EventMapper {
    fn execute(program_registry: &Arc<ProgramRegistry>, current_events: &EventBuffer) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let and_registry = get_and_registry(program_registry);
        event_buffer.extend(current_events.read().filter_map(|current_event| {
            and_registry.as_ref().get(current_event).cloned()
        }));

        let or_registry = get_or_registry(program_registry);
        event_buffer.extend(
            or_registry
                .as_ref()
                .iter()
                .filter(|(target_event, _)| {
                    !current_events.contains(target_event)
                })
                .map(|(_, new_event)| new_event.clone())
        );


        event_buffer
    }
}