use super::{content_from_selector, date_parser, file_parser, ParseInput};
use crate::downloaders::ModInfo;

/// Parses the name of the mod and the created and updated dates from the page HTML.
pub fn retrieve_mod_info(input: &ParseInput) -> Option<ModInfo> {
    let selectors = input.downloader.get_info_selectors();
    let mod_name = content_from_selector(&input, &selectors.name);
    let mod_dates = date_parser::retrieve_dates(input);
    let files = file_parser::retrieve_files(input);

    match mod_name.zip(mod_dates) {
        Some((name, (created, updated))) => Some(ModInfo {
            name,
            created,
            updated,
            files,
        }),
        None => None,
    }
}
