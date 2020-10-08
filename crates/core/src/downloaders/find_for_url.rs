use url::Url;

use super::{modthesims, Downloader};

/// Defines the different results from the find operation.
pub enum FindResult {
    InvalidUrl,
    UnrecognizedUrl,
    Found(Box<dyn Downloader>),
}

/// Returns the correct selector for the given URL, or an error in case the URL is malformed or not
/// recognized.
pub fn find_for_url(url: &str) -> FindResult {
    if !is_valid_url(url) {
        return FindResult::InvalidUrl;
    }

    let downloader = get_all_downloaders()
        .into_iter()
        .find(move |downloader| find_in_matchers(url, &downloader.hostname_matchers()));

    match downloader {
        Some(d) => FindResult::Found(d),
        None => FindResult::UnrecognizedUrl,
    }
}

fn find_in_matchers(url: &str, matchers: &[String]) -> bool {
    let parsed_url = Url::parse(url).unwrap();
    let domain = parsed_url.domain();

    if let Some(domain) = domain {
        matchers.iter().any(|matcher| domain == matcher)
    } else {
        false
    }
}

fn get_all_downloaders() -> Vec<Box<dyn Downloader>> {
    vec![Box::new(modthesims::ModTheSimsDownloader::default())]
}

fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}
