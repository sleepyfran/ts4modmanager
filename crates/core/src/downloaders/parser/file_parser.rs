use boolinator::Boolinator;
use scraper::{ElementRef, Selector};
use url::Url;

use super::{content_from_element, ParseInput};
use crate::downloaders::File;

/// Parses the created and updated dates from the page.
pub fn retrieve_files(input: &ParseInput) -> Vec<File> {
    let files_selectors = input.downloader.get_download_selector();
    let selector = Selector::parse(&files_selectors.selector).unwrap();
    let mut element = input.html.select(&selector);

    match element.next() {
        Some(element) => {
            if files_selectors.contained_in_table {
                retrieve_files_from_table(element)
            } else {
                retrieve_single_file(element)
                    .map(|file| vec![file])
                    .or_else(|| Some(vec![]))
                    .unwrap()
            }
        }
        None => vec![],
    }
}

fn retrieve_single_file(element: ElementRef) -> Option<File> {
    let file = try_get_file_info(element);

    file.or_else(|| {
        let children = element.children();

        for child in children {
            let found_file = ElementRef::wrap(child).and_then(retrieve_single_file);
            if found_file.is_some() {
                return found_file;
            }
        }

        None
    })
}

fn retrieve_files_from_table(element: ElementRef) -> Vec<File> {
    let rows_selector = Selector::parse("tbody > tr > td").unwrap();
    let rows = element.select(&rows_selector);

    rows.filter_map(retrieve_single_file).collect()
}

fn try_get_file_info(element: ElementRef) -> Option<File> {
    let href_attr = element.value().attr("href");
    match href_attr {
        Some(url) => valid_url(url).as_option().map(|_| File {
            name: content_from_element(element),
            url: url.to_string(),
        }),
        None => None,
    }
}

fn valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}
