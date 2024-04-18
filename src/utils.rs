use futures::stream::TryStreamExt;
use rspotify::{clients::pagination::Paginator, ClientResult};

pub async fn fetch_all<T>(paginator: Paginator<'_, ClientResult<T>>) -> Vec<T> {
    paginator.try_collect::<Vec<_>>().await.unwrap()
}
