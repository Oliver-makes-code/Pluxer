use std::sync::Arc;

use pluxer_backend::fluxer::{
    FluxerApi, fluxer_core::{
        Client, Error,
        client::{ClientOptions, typed_events::DispatchEvent},
    }, fluxer_rest::RestOptions, fluxer_types::{ApiInstance, ApiUser, Routes},
};

use crate::bot::PluxerContext;

pub async fn run(api_url: &str, token: &str, instance_name: Arc<str>) -> Result<(), Error> {
    let options = ClientOptions {
        intents: 0,
        wait_for_guilds: true,
        rest: Some(RestOptions {
            api_url: api_url.into(),
            ..Default::default()
        }),

        ..Default::default()
    };

    let mut client = Client::new(options);

    let context = Arc::new(PluxerContext::<FluxerApi>::new(client.rest.clone()));

    client.on_typed(move |event| {
        let context = context.clone();
        let instance_name = instance_name.clone();

        async move {
            match event {
                DispatchEvent::Ready => {
                    let user = context.bot.get::<ApiUser>(Routes::current_user()).await.unwrap();

                    println!("{}: User '{}' ({}) ready!", instance_name, user.username, user.id);
                }

                DispatchEvent::MessageCreate { message, .. } => {
                    context.on_message(&message).await.unwrap();
                }

                _ => {}
            }
        }
    });

    return client.login(token).await;
}
