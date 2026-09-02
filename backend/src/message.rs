use crate::PluxerApi;

pub trait BackendMessage: Send + Sync {
    type Api: PluxerApi;

    fn id(&self) -> &<Self::Api as PluxerApi>::Id;

    fn channel_id(&self) -> &<Self::Api as PluxerApi>::Id;

    fn author(&self) -> &<Self::Api as crate::PluxerApi>::User;

    fn content(&self) -> &str;

    fn created_by_bot(&self) -> bool;

    fn attachments(&self) -> Vec<String>;
}
