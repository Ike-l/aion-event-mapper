use aion_event::prelude::{Event, EventBuffer};

pub struct AndMapping {
    requires: Event,
    spawns: Event,
}

impl AndMapping {
    pub fn is_satisfied(&self, events: &EventBuffer) -> bool {
        events.contains(&self.requires) 
    }

    pub fn spawns(&self) -> &Event {
        &self.spawns
    }
}