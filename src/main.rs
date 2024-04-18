use anyhow::Result;

mod api_client;
mod config;
mod error;
mod handler;

use futures::stream::TryStreamExt;
use handler::ExportHandler;
use librespot::{
    core::{config::SessionConfig, session::Session},
    discovery::Credentials,
    protocol::authentication::AuthenticationType,
};
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::SimplifiedPlaylist,
};

use inquire::{MultiSelect, Select};

#[tokio::main]
async fn main() -> Result<()> {
    let spotify_api = api_client::get_client().await?;
    let api_clone = spotify_api.clone();

    let fetch_playlists = tokio::spawn(async move {
        let mut playlist_stream = api_clone.current_user_playlists();
        let mut playlists = Vec::new();
        while let Some(playlist) = playlist_stream
            .try_next()
            .await
            .expect("Error fetching next playlist")
        {
            playlists.push(playlist)
        }
        playlists
    });

    let token = spotify_api
        .get_token()
        .lock()
        .await
        .expect("Couldn't get lock")
        .clone()
        .expect("Failed to clone token")
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

    let menu_options = vec!["Export library metadata", "Export library audio"];
    let intent = Select::new("What do you want to do?", menu_options).prompt();

    let playlists = fetch_playlists.await?;
    let mut playlist_names: Vec<String> = playlists
        .clone()
        .into_iter()
        .map(|playlist| playlist.name)
        .collect();
    playlist_names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    let ans = MultiSelect::new("Select playlist(s):", playlist_names.clone())
        .with_page_size(8)
        .prompt()
        .expect("Error parsing selection.");

    let selected_playlists: Vec<SimplifiedPlaylist> = playlists
        .into_iter()
        .filter(|playlist| ans.contains(&playlist.name))
        .collect();

    let handler = ExportHandler::new(
        spotify_api,
        _session,
        selected_playlists,
        config::get_config_directory(),
    );
    match intent {
        Ok("Export library metadata") => {
            handler.get_metadata().await;
        }
        Ok("Export library audio") => {}
        _ => {
            eprintln!("Error determining intent. Exiting.");
        }
    }

    Ok(())
}
