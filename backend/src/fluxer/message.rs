use fluxer_core::Message;
use fluxer_types::Snowflake;

use crate::{
    fluxer::FluxerApi,
    message::{BackendMessage, FileAttachment, ReferencedMessageKind},
    user::BackendUser,
};

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

    fn referenced_message(&self) -> Option<&Message> {
        return self.referenced_message.as_deref();
    }

    fn referenced_message_kind(&self) -> Option<ReferencedMessageKind> {
        let reference = self.message_reference.as_ref()?;

        let kind = reference.kind?;

        if kind == 0 {
            return Some(ReferencedMessageKind::Reply);
        }

        if kind == 1 {
            return Some(ReferencedMessageKind::Forward);
        }

        return None;
    }

    fn attachments(&self) -> impl Iterator<Item = FileAttachment> {
        return self.attachments.iter().filter_map(|it| {
            Some(FileAttachment {
                file_name: it.filename.clone(),
                file_url: it.url.clone()?,
            })
        });
    }
}
