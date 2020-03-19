use bytes::buf::BufExt as _;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use std::collections::HashSet;

pub(super) async fn add_words(
    words: &mut HashSet<String>,
    uri: &Uri,
) -> Result<usize, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get(uri.clone()).await?;

    let body = hyper::body::aggregate(res).await?;

    // TODO perhaps check res.status() before parsing? find a way to skip
    // individual allocations
    let web_words: Vec<String> = serde_json::from_reader(body.reader())?;

    let inserted = web_words.len();

    for word in web_words {
        words.insert(word);
    }

    Ok(inserted)
}
