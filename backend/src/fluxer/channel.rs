use fluxer_core::Channel;

use crate::{channel::BackendChannel, fluxer::FluxerApi};

impl BackendChannel for Channel {
    type Api = FluxerApi;

    fn id(&self) -> &<Self::Api as crate::PluxerApi>::Id {
        return &self.id;
    }

    fn is_message_channel(&self) -> bool {
        return self.is_text();
    }
}
