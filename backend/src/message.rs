use crate::PluxerApi;

pub trait BackendMessage {
    type Api: PluxerApi;

    fn id(&self) -> &<Self::Api as PluxerApi>::Id;
    fn channel_id(&self) -> Option<&<Self::Api as PluxerApi>::Id>;

    fn content(&self) -> &str;

    fn created_by_bot(&self) -> bool;
}
