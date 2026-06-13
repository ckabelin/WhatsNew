use reqwest::{Client, StatusCode};

use crate::error::Result;

pub const USER_AGENT: &str = concat!(
    "WhatsNew/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/ckabelin/WhatsNew)"
);

/// Builds the shared HTTP client used for feed fetching and autodiscovery.
pub fn build_client() -> reqwest::Result<Client> {
    Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(20))
        .build()
}

/// The result of fetching a feed URL with conditional-GET headers.
pub struct FetchedFeed {
    /// `None` if the server responded `304 Not Modified`.
    pub body: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub status: StatusCode,
}

/// Fetches `url`, sending `If-None-Match`/`If-Modified-Since` headers when prior
/// caching info is available so unchanged feeds don't transfer a full body.
pub async fn fetch_feed(
    client: &Client,
    url: &str,
    etag: Option<&str>,
    last_modified: Option<&str>,
) -> Result<FetchedFeed> {
    let mut req = client.get(url);
    if let Some(etag) = etag {
        req = req.header(reqwest::header::IF_NONE_MATCH, etag);
    }
    if let Some(last_modified) = last_modified {
        req = req.header(reqwest::header::IF_MODIFIED_SINCE, last_modified);
    }

    let resp = req.send().await?;
    let status = resp.status();
    let etag = header_str(&resp, reqwest::header::ETAG);
    let last_modified = header_str(&resp, reqwest::header::LAST_MODIFIED);

    if status == StatusCode::NOT_MODIFIED {
        return Ok(FetchedFeed {
            body: None,
            etag,
            last_modified,
            status,
        });
    }

    let resp = resp.error_for_status()?;
    let body = resp.text().await?;
    Ok(FetchedFeed {
        body: Some(body),
        etag,
        last_modified,
        status,
    })
}

/// Fetches `url` and returns the raw response body as text, used for feed
/// autodiscovery (fetching a site's HTML to look for `<link rel=alternate>` tags).
pub async fn fetch_html(client: &Client, url: &str) -> Result<String> {
    let resp = client.get(url).send().await?.error_for_status()?;
    Ok(resp.text().await?)
}

fn header_str(resp: &reqwest::Response, name: reqwest::header::HeaderName) -> Option<String> {
    resp.headers()
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}
