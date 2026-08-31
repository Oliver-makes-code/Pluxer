use fluxer_core::{Channel, Error, Message, User};
use fluxer_rest::Rest;
use fluxer_types::Snowflake;

use crate::PluxerApi;

pub use fluxer_builders;
pub use fluxer_core;
pub use fluxer_rest;
pub use fluxer_types;
pub use fluxer_util;
pub use fluxer_ws;

mod bot;
mod channel;
mod id;
mod message;
mod user;

pub struct FluxerApi;

impl PluxerApi for FluxerApi {
    type Error = Error;
    type Bot = Rest;
    type Id = Snowflake;
    type Message = Message;
    type User = User;
    type Channel = Channel;
}
