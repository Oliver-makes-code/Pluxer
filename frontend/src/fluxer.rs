use fluxer_core::{
    Client,
    client::{ClientOptions, typed_events::DispatchEvent},
};
use fluxer_rest::RestOptions;
use pluxer_backend::fluxer::FluxerApi;

use crate::bot::on_message;

pub async fn run(api_url: &str, token: &str) {
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
    let rest = client.rest.clone();

    client.on_typed(move |event| {
        let rest = rest.clone();
        async move {
            match event {
                DispatchEvent::Ready => {
                    println!("Ready!");
                }

                DispatchEvent::MessageCreate { message, .. } => {
                    on_message::<FluxerApi>(&rest, &message).await.unwrap();
                }
                _ => {}
            }
        }
    });

    if let Err(e) = client.login(token).await {
        eprintln!("{}", e);
    }
}
