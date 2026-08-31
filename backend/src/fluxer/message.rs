use fluxer_core::Message;
use fluxer_types::Snowflake;

use crate::{fluxer::FluxerApi, message::BackendMessage, user::BackendUser};

impl BackendMessage for Message {
    type Api = FluxerApi;

    fn id(&self) -> &Snowflake {
        return &self.id;
    }

    fn content(&self) -> &str {
        return &self.content;
    }

    fn created_by_bot(&self) -> bool {
        return self.webhook_id.is_some() || self.author.is_bot();
    }

    fn channel_id(&self) -> Option<&Snowflake> {
        return Some(&self.channel_id);
    }
}
