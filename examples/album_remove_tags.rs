use dotenv::dotenv;
use lastfm_rust::{APIResponse, Lastfm};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY env variable is required");
    let api_secret = std::env::var("API_SECRET").expect("API_SECRET env variable is required");
    let sk = std::env::var("SK").expect("SK env variable is required");

    let lastfm = Lastfm::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .session_key(sk)
        .build()?;

    let _sk = lastfm.auth();
    // lastfm.set_sk(sk);

    let response = lastfm
        .album()
        .remove_tag()
        .artist("Billie Eilish")
        .album("HIT ME HARD AND SOFT")
        .tag("aoty")
        .send()
        .await?;

    match response {
        APIResponse::Success(value) => {
            println!("{}", value);
        }
        APIResponse::Error(err) => {
            println!("Error: {} - {}", err.error, err.message);
        }
    }

    Ok(())
}
