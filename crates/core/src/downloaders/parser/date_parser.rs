use chrono::prelude::*;
use regex::Regex;

use super::{content_from_selector, ParseInput};
use crate::downloaders::{DateInfoSelectors, SameDateInfoSelectors};

type Date = NaiveDateTime;
type Dates = Option<(Date, Date)>;

/// Parses the created and updated dates from the page.
pub fn retrieve_dates(input: &ParseInput) -> Dates {
    let selectors = input.downloader.get_date_selectors();

    match selectors {
        DateInfoSelectors::Same(selectors) => retrieve_same_element_dates(input, selectors),
        DateInfoSelectors::Different(_selectors) => panic!("Not implemented yet"),
    }
}

fn retrieve_same_element_dates(input: &ParseInput, selectors: SameDateInfoSelectors) -> Dates {
    let dates_content = content_from_selector(&input, &selectors.selector);
    let created_date = dates_content
        .clone()
        .map(|content| retrieve_date_from(input, &content, &selectors.created_regex))
        .flatten();
    let updated_date = dates_content
        .map(|content| retrieve_date_from(input, &content, &selectors.updated_regex))
        .flatten();

    created_date.zip(updated_date.or(created_date))
}

fn retrieve_date_from(input: &ParseInput, content: &str, regex: &str) -> Option<Date> {
    Regex::new(regex)
        .unwrap()
        .captures(content)
        .and_then(|captures| captures.get(1))
        .map(|regex_match| regex_match.as_str())
        .and_then(|date| input.downloader.parse_date(date))
}
