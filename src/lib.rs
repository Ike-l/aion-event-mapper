pub mod event_mapper;

pub mod prelude {
    pub use super::{
        event_mapper::{
            EventMapper,
        }
    };

    #[cfg(feature = "and-mapping")]
    pub use super::{
        event_mapper::{
            get_and_mappings::{
                GetAndMappings
            },
            and_mapping::{
                AndMapping
            }
        }
    };

    #[cfg(feature = "or-mapping")]
    pub use super::{
        event_mapper::{
            get_or_mappings::{
                GetOrMappings
            },
            or_mapping::{
                OrMapping
            }
        }
    };
}

