use std::sync::Arc;

use aion_event::prelude::{EventSystem, EventBuffer};

use aion_program::prelude::ProgramRegistry;

pub struct EventMapper {

}

impl EventSystem for EventMapper {
    fn execute(program_registry: &Arc<ProgramRegistry>, current_events: &EventBuffer) -> EventBuffer {
        
    }
}