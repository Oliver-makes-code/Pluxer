use fluxer_core::Webhook;

use crate::{fluxer::FluxerApi, webhook::BackendWebhook};

impl BackendWebhook for Webhook {
    type Api = FluxerApi;

    fn owner(&self) -> &<Self::Api as crate::PluxerApi>::User {
        return &self.user;
    }
}
