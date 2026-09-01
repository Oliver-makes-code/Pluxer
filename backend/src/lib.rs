use std::error::Error;

use crate::{
    bot::BackendBot, channel::BackendChannel, id::BackendId, message::BackendMessage,
    user::BackendUser, webhook::BackendWebhook,
};

pub mod bot;
pub mod channel;
pub mod embed;
pub mod id;
pub mod message;
pub mod user;
pub mod webhook;

#[cfg(feature = "fluxer")]
pub mod fluxer;

pub trait PluxerApi: Send + Sync {
    type Error: Error + Send + Sync + 'static;
    type Id: BackendId<Api = Self>;
    type Bot: BackendBot<Api = Self>;
    type User: BackendUser<Api = Self>;
    type Message: BackendMessage<Api = Self>;
    type Channel: BackendChannel<Api = Self>;
    type Webhook: BackendWebhook<Api = Self>;
}
