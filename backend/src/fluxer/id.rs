use fluxer_types::Snowflake;
use pluxer_database::platform_id::PlatformId;

use crate::{fluxer::FluxerApi, id::BackendId};

impl BackendId for Snowflake {
    type Api = FluxerApi;

    fn to_platform_id(&self, instance_name: Option<&str>) -> PlatformId {
        return PlatformId::Fluxer {
            snowflake: self.parse().unwrap(),
            instance_name: instance_name.unwrap().into(),
        };
    }
}
