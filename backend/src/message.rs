use crate::PluxerApi;

pub struct FileAttachment {
    pub file_name: String,
    pub file_url: String,
}

pub trait BackendMessage: Send + Sync {
    type Api: PluxerApi;

    fn id(&self) -> &<Self::Api as PluxerApi>::Id;

    fn channel_id(&self) -> &<Self::Api as PluxerApi>::Id;

    fn author(&self) -> &<Self::Api as PluxerApi>::User;

    fn content(&self) -> &str;

    fn created_by_bot(&self) -> bool;

    fn referenced_message(&self) -> Option<&<Self::Api as PluxerApi>::Message>;

    fn attachments(&self) -> impl Iterator<Item = FileAttachment>;
}
