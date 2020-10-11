use console::style;
use dialoguer::Confirm;
use seahorse::{Command, Context};

use core::downloaders;
use core::downloaders::{DownloadResult, Downloader, FindResult, ModInfo, ParseResult};

use crate::emoji;
use crate::io;

/// Creates a command that install a mod.
pub fn create() -> Command {
    Command::new("install")
        .alias("i")
        .usage("[url] Installs a mod from a URL")
        .action(handler)
}

fn handler(context: &Context) {
    if let Some(url) = context.args.first() {
        attempt_to_find_downloader_for(url);
    } else {
        io::show_error(emoji::for_error(), "No URL specified");
        io::show_info(
            emoji::for_sucess(),
            "Usage: ts4mods install https://test.com/mod",
        )
    }
}

fn attempt_to_find_downloader_for(url: &str) {
    io::show_info(
        emoji::for_search(),
        "Attempting to find a downloader for the given URL",
    );

    let downloader = downloaders::find_for_url(url);
    match downloader {
        FindResult::InvalidUrl => io::show_error(emoji::for_error(), "Invalid URL"),
        FindResult::UnrecognizedUrl => io::show_error(
            emoji::for_unsuccessful(),
            "We couldn't find any downloader for your URL",
        ),
        FindResult::Found(downloader) => fetch_page(url, &*downloader),
    }
}

fn fetch_page(url: &str, downloader: &dyn Downloader) {
    io::show_info(
        emoji::for_fetching(),
        format!("URL matched for {}, fetching page...", downloader.name()),
    );

    let page_content = downloaders::download_page(url);
    match page_content {
        DownloadResult::Unknown(_) => io::show_error(
            emoji::for_error(),
            "There was an error loading the page. Is your internet working?",
        ),
        DownloadResult::NotFound => io::show_error(
            emoji::for_error(),
            "The URL returned a 404, which means not found. Did you copy the correct URL?",
        ),
        DownloadResult::Success(content) => parse_page(&content, downloader),
        _ => io::show_error(
            emoji::for_error(),
            "There was some HTTP error, maybe try again?",
        ),
    }
}

fn parse_page(content: &str, downloader: &dyn Downloader) {
    io::show_info(emoji::for_parsing(), "Parsing page content...");

    let mod_info = downloaders::parse_mod_info(content, downloader);
    match mod_info {
        ParseResult::ErrorRetrievingInfo => io::show_error(emoji::for_error(), "Unable to correctly parse the page content. If you believe this is a bug, please report it"),
        ParseResult::ErrorRetrievingFiles => io::show_error(emoji::for_error(), "Unable to retrieve any files. Is the URL pointing to a mod with downloadable content? If you believe this is a bug, please report it"),
        ParseResult::Success(mod_info) => ask_confirmation(mod_info),
    }
}

fn ask_confirmation(mod_info: ModInfo) {
    io::show_success(
        emoji::for_parsing(),
        format!(
            "Finished parsing the page. Found the mod {}, updated {} with {} file(s)",
            style(&mod_info.name).blue(),
            style(&mod_info.updated).blue(),
            style(&mod_info.files.len()).blue()
        ),
    );

    let confirmed = Confirm::new()
        .with_prompt(io::text_for_info(
            emoji::for_question(),
            format!("Are you sure you want to install {}?", mod_info.name),
        ))
        .interact()
        .unwrap_or_default();
    if confirmed {
        io::show_info(emoji::for_fetching(), "Coming soon")
    } else {
        io::show_info(emoji::for_warning(), "Okay, nothing was installed")
    };
}
