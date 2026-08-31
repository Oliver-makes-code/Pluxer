use crate::PluxerApi;

pub trait BackendChannel: Send + Sync {
    type Api: PluxerApi;

    fn id(&self) -> &<Self::Api as PluxerApi>::Id;

    /// Returns true when messages can be sent in the channel.
    fn is_message_channel(&self) -> bool;
}
