use anyhow::Result;
use rspotify::clients::OAuthClient;

mod client;
mod config;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    let spotify = client::get_client().await?;

    let user = spotify.current_user().await?;

    println!("User: {:?}", user);

    Ok(())
}
