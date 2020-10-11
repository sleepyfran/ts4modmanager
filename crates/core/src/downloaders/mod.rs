mod download_page;
mod find_for_url;
mod modthesims;
mod parser;

use chrono::NaiveDateTime;

pub use download_page::*;
pub use find_for_url::*;
pub use parser::*;

pub struct SameDateInfoSelectors {
    /// Regex to parse the created date.
    created_regex: String,
    /// Regex to parse the updated date.
    updated_regex: String,
    /// Selector that has both
    selector: String,
}

pub struct DifferentDateInfoSelectors {
    /// Regex to parse the created date.
    created_regex: String,
    /// Selector of the created date.
    created_selector: String,
    /// Selector of the updated date.
    updated_selector: String,
    /// Regex to parse the updated date.
    updated_regex: String,
}

/// Collects all selectors for the date information about updates.
pub enum DateInfoSelectors {
    /// Case in which both the created and updated date are contained in the same container.
    Same(SameDateInfoSelectors),

    /// Case in which they're in different containers.
    Different(DifferentDateInfoSelectors),
}

/// Collects all the selectors for the general information of the mod.
pub struct ModInfoSelectors {
    /// Selector for the name of the mod.
    pub name: String,
}

/// Defines the behavior that all downloaders should adhere to.
pub trait Downloader {
    /// Name of the webpage that we're trying to download the mod from.
    fn name(&self) -> String;

    /// Returns all the URL host names that match this downloader
    fn hostname_matchers(&self) -> Vec<String>;

    /// Returns all the selectors that allow to query the mod information.
    fn get_info_selectors(&self) -> ModInfoSelectors;
    fn get_date_selectors(&self) -> DateInfoSelectors;
    fn get_download_selector(&self) -> String;

    /// Function called by the date parser after matching the regex given by this same downloader.
    fn parse_date(&self, date: &str) -> Option<NaiveDateTime>;
}

/// Defines all the information needed regarding a mod after it's been parsed.
pub struct ModInfo {
    /// Name of the mod.
    pub name: String,
    /// Date when the mod was first uploaded.
    pub created: NaiveDateTime,
    /// Date of the last update of the mod.
    pub updated: NaiveDateTime,
}
