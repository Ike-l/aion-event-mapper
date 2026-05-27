use std::sync::Arc;

use aion_ecs::prelude::Query;
use aion_program::prelude::{ProgramRegistry, ProgramRegistryResolveEitherError};
use tokio::runtime::Runtime;

use crate::prelude::OrMapping;


pub trait GetOrMappings {
    fn get_or_mappings(
        &self,
        runtime: Option<&Runtime>
    ) -> Result<Query<'_, &OrMapping>, ProgramRegistryResolveEitherError>;
}

impl GetOrMappings for Arc<ProgramRegistry> {
    fn get_or_mappings(
        &self,
        runtime: Option<&Runtime>
    ) -> Result<Query<'_, &OrMapping>, ProgramRegistryResolveEitherError>
    {
        self.resolve_simple_either::<Query<&OrMapping>>(runtime)
    }
}