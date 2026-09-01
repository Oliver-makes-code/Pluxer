use fluxer_core::Webhook;

use crate::{fluxer::FluxerApi, webhook::BackendWebhook};

impl BackendWebhook for Webhook {
    type Api = FluxerApi;
}
