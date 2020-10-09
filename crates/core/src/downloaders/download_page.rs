use minreq::Error;

use super::Downloader;

/// Defines the different result for the download page operation.
pub enum DownloadResult {
    Unknown(Error),
    NotFound,
    HttpError(i32),
    StringTransformationError,
    Success(String),
}

/// Downloads the page specified in the downloader's URL and transforms it into a string containing
/// the HTML.
pub fn download_page(downloader: &dyn Downloader) -> DownloadResult {
    let response = minreq::get(downloader.get_url()).send();

    if let Ok(response) = response {
        match response.status_code {
            200..=299 => to_download_result(response.as_str()),
            404 => DownloadResult::NotFound,
            _ => DownloadResult::HttpError(response.status_code),
        }
    } else {
        DownloadResult::Unknown(response.unwrap_err())
    }
}

fn to_download_result(res: Result<&str, Error>) -> DownloadResult {
    match res {
        Ok(res) => DownloadResult::Success(res.to_string()),
        Err(_) => DownloadResult::StringTransformationError,
    }
}
