pub mod event_mapper;

pub mod prelude {
    pub use super::{
        event_mapper::{
            EventMapper,
            and_registry::{
                AND_REGISTRY_ACCESS_BUILDER,
                AND_REGISTRY_RESOURCE_ID,
                AndRegistry,
                get_and_registry
            },
            or_registry::{
                OR_REGISTRY_ACCESS_BUILDER,
                OR_REGISTRY_RESOURCE_ID,
                OrRegistry,
                get_or_registry
            }
        }
    };
}

