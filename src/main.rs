mod api_client;
mod config;
mod downloader;
mod error;

use anyhow::Result;
use librespot::{
    core::{config::SessionConfig, session::Session, spotify_id::SpotifyId},
    discovery::Credentials,
    metadata::{Metadata, Playlist},
    protocol::authentication::AuthenticationType,
};
use rspotify::clients::BaseClient;

#[tokio::main]
async fn main() -> Result<()> {
    let spotify = api_client::get_client().await?;
    println!("> Spotify API loaded");

    let token = spotify
        .get_token()
        .lock()
        .await
        .unwrap()
        .clone()
        .unwrap()
        .access_token;

    let creds = Credentials {
        username: config::Config::get_username_from_config().unwrap(),
        auth_type: AuthenticationType::AUTHENTICATION_SPOTIFY_TOKEN,
        auth_data: token.into(),
    };

    let session_config = SessionConfig::default();

    let session = Session::connect(session_config, creds, None, false)
        .await?
        .0;
    println!("> Spotify session connected");

    let playlist_uri = SpotifyId::from_uri("spotify:playlist:37i9dQZF1DWXRqgorJj26U").unwrap();

    let p = Playlist::get(&session, playlist_uri).await;

    println!("{:#?}", p);

    Ok(())
}
