mod find_for_url;
mod modthesims;

pub use find_for_url::{find_for_url, FindResult};

/// Different types of selectors for the created and updated dates.
pub enum CreateAndUpdateDateSelector {
    /// Case in which both the created and updated date are contained in the same container.
    Same {
        /// Regex to parse the date.
        regex: String,
        /// Selector that has both
        selector: String,
    },

    /// Case in which they're in different containers.
    Different {
        /// Regex to parse the created date.
        created_regex: String,
        /// Selector of the created date.
        created_selector: String,
        /// Separator between the dates, if any.
        separator: Option<String>,
        /// Selector of the updated date.
        updated_selector: String,
        /// Regex to parse the updated date.
        updated_regex: String,
    },
}

/// Collects all selectors for the date information about updates.
pub struct DateInfoSelectors {
    /// Format of the date to parse.
    pub format: String,

    /// Locale of the date to parse.
    pub locale: String,

    /// Selectors for the created and updated dates.
    pub selector: CreateAndUpdateDateSelector,
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
}
