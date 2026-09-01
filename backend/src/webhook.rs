use crate::PluxerApi;

pub trait BackendWebhook: Send + Sync + Clone + 'static {
    type Api: PluxerApi;

    fn owner(&self) -> &<Self::Api as PluxerApi>::User;
}
