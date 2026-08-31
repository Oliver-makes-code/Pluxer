use crate::{
    bot::BackendBot, channel::BackendChannel, id::BackendId, message::BackendMessage,
    user::BackendUser,
};

pub mod bot;
pub mod channel;
pub mod id;
pub mod message;
pub mod user;

#[cfg(feature = "fluxer")]
pub mod fluxer;

pub trait PluxerApi {
    type Error;
    type Id: BackendId<Api = Self>;
    type Bot: BackendBot<Api = Self>;
    type User: BackendUser<Api = Self>;
    type Message: BackendMessage<Api = Self>;
    type Channel: BackendChannel<Api = Self>;
}
