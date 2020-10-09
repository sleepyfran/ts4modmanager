use chrono::prelude::*;

use super::{Downloader, ModInfo};

/// Defines the different results of the parse opperation.
pub enum ParseResult {
    ErrorRetrievingDate,
    ErrorRetrievingName,
    ErrorRetrievingFiles,
    Success(ModInfo),
}

/// Fetches the page content and attempts to parse the info of the mod from the given downloader.
pub fn parse_mod_info(html_content: String, downloader: &dyn Downloader) -> ParseResult {
    ParseResult::ErrorRetrievingDate
}
