use crate::PluxerApi;

pub trait BackendWebhook: Send + Sync {
    type Api: PluxerApi;
}
