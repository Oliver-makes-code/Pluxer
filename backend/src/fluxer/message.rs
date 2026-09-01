use fluxer_core::Message;
use fluxer_types::Snowflake;

use crate::{fluxer::FluxerApi, message::BackendMessage, user::BackendUser};

impl BackendMessage for Message {
    type Api = FluxerApi;

    fn id(&self) -> &Snowflake {
        return &self.id;
    }

    fn author(&self) -> &<Self::Api as crate::PluxerApi>::User {
        return &self.author;
    }

    fn content(&self) -> &str {
        return &self.content;
    }

    fn created_by_bot(&self) -> bool {
        return self.webhook_id.is_some() || self.author.is_bot();
    }

    fn channel_id(&self) -> &Snowflake {
        return &self.channel_id;
    }
}
