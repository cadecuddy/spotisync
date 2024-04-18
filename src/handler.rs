use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicI32, AtomicUsize, Ordering},
        Arc,
    },
};

use csv::Writer;
use futures::TryStreamExt;
use librespot::core::session::Session;
use rspotify::{
    clients::BaseClient,
    model::{PlayableItem, SimplifiedPlaylist},
    AuthCodeSpotify,
};
use spinners::Spinner;

pub struct ExportHandler {
    pub api_client: AuthCodeSpotify,
    pub session: Session,
    pub playlists: Vec<SimplifiedPlaylist>,
    pub config_path: PathBuf,
}

impl ExportHandler {
    pub fn new(
        api_client: AuthCodeSpotify,
        session: Session,
        playlists: Vec<SimplifiedPlaylist>,
        config_path: PathBuf,
    ) -> ExportHandler {
        ExportHandler {
            api_client,
            session,
            playlists,
            config_path,
        }
    }

    pub async fn get_metadata(&self) {
        let mut handles = Vec::new();
        let total_tracks = Arc::new(AtomicI32::new(0));

        let mut sp = Spinner::new(spinners::Spinners::Flip, "Saving metadata".to_string());
        for playlist in &self.playlists {
            let p = playlist.clone();
            let api_clone = self.api_client.clone();
            let path_clone = self.config_path.clone();
            let total_tracks_clone = total_tracks.clone();

            handles.push(tokio::spawn(async move {
                let mut track_stream = api_clone.playlist_items(p.id, None, None);

                let filename = p
                    .name
                    .chars()
                    .map(|c| match c {
                        ' ' => '_',
                        '/' => '-',
                        _ => c,
                    })
                    .collect::<String>();
                let mut writer = Writer::from_path(path_clone.join(format!("{}.csv", filename)))
                    .expect(format!("Couldn't create CSV writer for {}", filename).as_str());

                writer
                    .write_record(vec!["track name", "album name", "artist name(s)"])
                    .expect("Error writing headers");

                while let Some(item) = track_stream
                    .try_next()
                    .await
                    .expect("Couldn't get next track")
                {
                    match item.track {
                        Some(PlayableItem::Track(track)) => {
                            let mut artists = String::new();
                            for artist in track.artists {
                                artists.push_str(&artist.name);
                                artists.push_str(", ");
                            }
                            let record = vec![
                                track.name,
                                track.album.name,
                                artists.trim_end_matches(", ").to_string(),
                            ];
                            writer.write_record(record).expect("Failed to write record");
                        }
                        _ => {}
                    }
                }

                writer.flush().expect("Failed to flush CSV writer");
                total_tracks_clone.fetch_add(p.tracks.total as i32, Ordering::SeqCst);
            }));
        }

        futures::future::join_all(handles).await;
        sp.stop();
        println!(
            "\nSaved metadata for {:?} tracks across {:?} playlists.",
            total_tracks,
            self.playlists.len()
        );
    }
}
