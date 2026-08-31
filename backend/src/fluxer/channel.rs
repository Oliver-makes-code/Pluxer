use fluxer_core::Channel;

use crate::{channel::BackendChannel, fluxer::FluxerApi};

impl BackendChannel for Channel {
    type Api = FluxerApi;
}
