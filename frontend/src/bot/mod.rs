use pluxer_backend::{PluxerApi, bot::BackendBot, message::BackendMessage};

pub async fn on_message<A: PluxerApi>(bot: &A::Bot, message: &A::Message) -> Result<(), A::Error> {
    if message.created_by_bot() {
        return Ok(());
    }

    let Some(channel_id) = message.channel_id() else {
        return Ok(());
    };

    if message.content() != "yip" {
        return Ok(());
    }

    bot.send_message(channel_id, "yap").await?;

    return Ok(());
}
