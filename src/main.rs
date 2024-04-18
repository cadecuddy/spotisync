use anyhow::Result;

mod api_client;
mod config;
mod downloader;
mod error;

use futures::stream::TryStreamExt;
use librespot::{
    core::{config::SessionConfig, session::Session},
    discovery::Credentials,
    protocol::authentication::AuthenticationType,
};
use rspotify::clients::{BaseClient, OAuthClient};

use inquire::MultiSelect;

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
    let mut playlist_names = Vec::new();
    while let Some(playlist) = playlists.try_next().await.unwrap() {
        playlist_names.push(playlist.name);
    }
    playlist_names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    let ans = MultiSelect::new("Select playlists to save:", playlist_names)
        .with_page_size(8)
        .prompt();

    match ans {
        Ok(answer) => println!("> Playlists selected: {:?}", answer),
        Err(_) => println!("> No playlists selected"),
    }

    Ok(())
}
