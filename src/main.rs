use anyhow::Result;

mod api_client;
mod config;
mod downloader;
mod error;

use futures::{pin_mut, stream::TryStreamExt};
use librespot::{
    core::{config::SessionConfig, session::Session},
    discovery::Credentials,
    protocol::authentication::AuthenticationType,
};
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::SimplifiedPlaylist,
};

#[tokio::main]
async fn main() -> Result<()> {
    let spotify_api = api_client::get_client().await?;
    println!("> Spotify API loaded");

    let token = spotify_api
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

    let _session = Session::connect(session_config, creds.clone(), None, false)
        .await?
        .0;
    println!("> Spotify session connected");

    let mut playlists = spotify_api.current_user_playlists();
    while let Some(playlist) = playlists.try_next().await.unwrap() {
        println!("> Playlist found: {}", playlist.name);
    }

    Ok(())
}
