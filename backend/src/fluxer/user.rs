use fluxer_core::User;
use fluxer_types::Snowflake;

use crate::{fluxer::FluxerApi, user::BackendUser};

impl BackendUser for User {
    type Api = FluxerApi;

    fn id(&self) -> &Snowflake {
        return &self.id;
    }

    fn is_bot(&self) -> bool {
        return self.bot;
    }
}
