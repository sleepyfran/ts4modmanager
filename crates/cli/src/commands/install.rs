use seahorse::{Command, Context};

use core::downloaders;
use core::downloaders::{Downloader, FindResult};

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
            emoji::for_info(),
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
        FindResult::Found(downloader) => install_with(downloader),
    }
}

fn install_with(downloader: Box<dyn Downloader>) {
    io::show_info(
        emoji::for_fetching(),
        format!(
            "URL matched for {}, fetching page and parsing...",
            downloader.name()
        ),
    );
}
