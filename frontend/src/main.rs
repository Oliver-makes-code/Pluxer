use dotenv::dotenv;

mod bot;
mod fluxer;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let api_url =
        std::env::var("FLUXER_API_ENDPOINT").unwrap_or("https://api.fluxer.app/v1".into());

    let token = std::env::var("FLUXER_BOT_TOKEN").unwrap();

    fluxer::run(&api_url, &token).await;
}
