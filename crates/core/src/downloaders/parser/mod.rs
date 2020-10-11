mod date_parser;
mod mod_info_parser;

use scraper::{Html, Selector};

use super::{Downloader, ModInfo};

/// Defines the different results of the parse opperation.
pub enum ParseResult {
    ErrorRetrievingInfo,
    ErrorRetrievingFiles,
    Success(ModInfo),
}

pub struct ParseInput<'a> {
    html: Html,
    downloader: &'a dyn Downloader,
}

/// Fetches the page content and attempts to parse the info of the mod from the given downloader.
pub fn parse_mod_info(html_content: &str, downloader: &dyn Downloader) -> ParseResult {
    let input = ParseInput {
        html: Html::parse_document(html_content),
        downloader,
    };
    let mod_info = mod_info_parser::retrieve_mod_info(&input);

    match mod_info {
        Some(info) => ParseResult::Success(info),
        None => ParseResult::ErrorRetrievingInfo,
    }
}

/// Retrieves the text content of an element if it exists.
fn content_from_selector(input: &ParseInput, selector: &str) -> Option<String> {
    let selector = Selector::parse(selector).unwrap();
    input
        .html
        .select(&selector)
        .next()
        .map(|element| element.text().collect::<String>())
        .map(|content| content.trim().replace("\n", " "))
}
