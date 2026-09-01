use std::sync::Arc;

use pluxer_backend::fluxer::{
    FluxerApi,
    fluxer_core::{
        Client,
        client::{ClientOptions, typed_events::DispatchEvent},
    },
    fluxer_rest::RestOptions,
    fluxer_types::{ApiUser, Routes},
};
use pluxer_database::sea_orm::DatabaseConnection;

use crate::bot::PluxerContext;

pub async fn run(
    api_url: Arc<str>,
    token: &str,
    instance_name: Arc<str>,
    database: DatabaseConnection,
) -> anyhow::Result<()> {
    let options = ClientOptions {
        intents: 0,
        wait_for_guilds: true,
        rest: Some(RestOptions {
            api_url: api_url.to_string(),
            ..Default::default()
        }),

        ..Default::default()
    };

    let mut client = Client::new(options);

    let context = Arc::new(
        PluxerContext::<FluxerApi>::new(
            client.rest.clone(),
            database,
            instance_name.clone(),
            api_url,
        )
        .await?,
    );

    client.on_typed(move |event| {
        let context = context.clone();
        let instance_name = instance_name.clone();

        async move {
            match event {
                DispatchEvent::Ready => {
                    let user = context
                        .bot
                        .get::<ApiUser>(Routes::current_user())
                        .await
                        .unwrap();

                    pawkit_logger::log!(
                        info,
                        "{}: User '{}' ({}) ready!",
                        instance_name,
                        user.username,
                        user.id
                    );
                }

                DispatchEvent::MessageCreate { message, .. } => {
                    context.on_message(&message).await.unwrap();
                }

                _ => {}
            }
        }
    });

    client.login(token).await?;

    return Ok(());
}
