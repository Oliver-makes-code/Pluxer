use fluxer_types::Snowflake;

use crate::{fluxer::FluxerApi, id::BackendId};

impl BackendId for Snowflake {
    type Api = FluxerApi;

    fn as_snowflake(&self) -> Option<u64> {
        return self.parse().ok();
    }
}
