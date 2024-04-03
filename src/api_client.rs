use anyhow::Result;
use rspotify::{
    clients::{BaseClient, OAuthClient},
    scopes, AuthCodeSpotify, Config as SpotifyConfig, Credentials, OAuth,
};

use crate::config::{self, *};

pub async fn check_token(spotify: &mut AuthCodeSpotify) -> Result<()> {
    match spotify.read_token_cache(true).await {
        Ok(Some(token)) => {
            let expired = token.is_expired();

            *spotify.get_token().lock().await.unwrap() = Some(token);

            if expired {
                match spotify.refetch_token().await? {
                    Some(token) => {
                        *spotify.get_token().lock().await.unwrap() = Some(token);
                        spotify.write_token_cache().await?;
                    }
                    None => {
                        println!("Token expired and failed to refresh");
                        prompt_user_for_token(spotify).await?;
                    }
                }
            }
        }
        _ => {
            println!("Token not found, prompting user for token");
            prompt_user_for_token(spotify).await?;
        }
    }

    Ok(())
}

pub async fn prompt_user_for_token(spotify: &mut AuthCodeSpotify) -> Result<()> {
    let url = spotify
        .get_authorize_url(false)
        .expect("Error getting auth url");
    spotify.prompt_for_token(&url).await?;

    Ok(())
}

pub async fn get_client() -> Result<AuthCodeSpotify> {
    let config = match Config::load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    let creds = Credentials::new(&config.client_id, &config.client_secret);

    let scopes = scopes!(
        "user-library-read",
        "playlist-read-private",
        "playlist-read-collaborative",
        "user-read-recently-played",
        "streaming"
    );

    let oauth = OAuth {
        redirect_uri: String::from("http://localhost:8069"),
        scopes,
        ..Default::default()
    };

    let spotify_config = SpotifyConfig {
        cache_path: config::get_config_directory().join("token.json"),
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };

    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, spotify_config);
    check_token(&mut spotify).await?;

    Ok(spotify)
}
