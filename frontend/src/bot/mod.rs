use pluxer_backend::{PluxerApi, bot::BackendBot, message::BackendMessage};

mod command;

pub struct PluxerContext<A: PluxerApi> {
    pub bot: A::Bot,
}

impl<A: PluxerApi> PluxerContext<A> {
    pub fn new(bot: A::Bot) -> Self {
        return Self { bot };
    }

    pub async fn on_message(&self, message: &A::Message) -> Result<(), A::Error> {
        if message.created_by_bot() {
            return Ok(());
        }

        let Some(channel_id) = message.channel_id() else {
            return Ok(());
        };

        if message.content() != "yip" {
            return Ok(());
        }

        self.bot.send_message(channel_id, "yap").await?;

        return Ok(());
    }
}
