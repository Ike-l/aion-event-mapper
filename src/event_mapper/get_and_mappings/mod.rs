use std::sync::Arc;

use aion_ecs::prelude::Query;
use aion_program::prelude::{ProgramRegistry, ProgramRegistryResolveEitherError};
use tokio::runtime::Runtime;

use crate::prelude::AndMapping;


pub trait GetAndMappings {
    fn get_and_mappings(
        &self,
        runtime: Option<&Runtime>
    ) -> Result<Query<'_, &AndMapping>, ProgramRegistryResolveEitherError>;
}

impl GetAndMappings for Arc<ProgramRegistry> {
    fn get_and_mappings(
        &self,
        runtime: Option<&Runtime>
    ) -> Result<Query<'_, &AndMapping>, ProgramRegistryResolveEitherError>
    {
        self.resolve_simple_either::<Query<&AndMapping>>(runtime)
    }
}