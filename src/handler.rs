use librespot::core::session::Session;
use rspotify::{clients::BaseClient, model::SimplifiedPlaylist, AuthCodeSpotify};

pub struct ExportHandler {
    pub api_client: AuthCodeSpotify,
    pub session: Session,
    pub playlists: Vec<SimplifiedPlaylist>,
}

impl ExportHandler {
    pub fn new(
        api_client: AuthCodeSpotify,
        session: Session,
        playlists: Vec<SimplifiedPlaylist>,
    ) -> ExportHandler {
        ExportHandler {
            api_client,
            session,
            playlists,
        }
    }

    pub async fn get_metadata(&self) {
        let mut handles = Vec::new();

        for playlist in &self.playlists {
            let p = playlist.clone();
            let api_clone = self.api_client.clone();
            handles.push(tokio::spawn(async move {
                let playlist = api_clone
                    .playlist(p.id, None, None)
                    .await
                    .expect(&format!("Failed to get playlist {}", p.name));
                println!("Playlist: {}", playlist.name);
                println!("Tracks: {}", playlist.tracks.total);
            }));
        }

        futures::future::join_all(handles).await;
    }
}
