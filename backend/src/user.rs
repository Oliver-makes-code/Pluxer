use crate::PluxerApi;

pub trait BackendUser: Send + Sync {
    type Api: PluxerApi;

    fn id(&self) -> &<Self::Api as PluxerApi>::Id;

    fn is_bot(&self) -> bool;
}
