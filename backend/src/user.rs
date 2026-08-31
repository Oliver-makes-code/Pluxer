use crate::PluxerApi;

pub trait BackendUser {
    type Api: PluxerApi;

    fn id(&self) -> &<Self::Api as PluxerApi>::Id;

    fn is_bot(&self) -> bool;
}
