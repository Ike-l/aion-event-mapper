use aion_event::prelude::{Event, EventBuffer};

pub struct OrMapping {
    requires_no: Event,
    spawns: Event,
}

impl OrMapping {
    pub fn is_satisfied(&self, events: &EventBuffer) -> bool {
        !events.contains(&self.requires_no) 
    }

    pub fn spawns(&self) -> &Event {
        &self.spawns
    }
}