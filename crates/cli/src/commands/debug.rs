use console::style;
use seahorse::{Command, Context};

use core::downloaders::{download_page, find_for_url, DownloadResult, FindResult};

use crate::{emoji, io};

/// Creates a command that helps with debugging a mod.
pub fn create() -> Command {
    Command::new("debug")
        .usage("Come on, you're the developer :)")
        .action(handler)
}

fn handler(context: &Context) {
    if context.args.is_empty() {
        println!("{}", style("No command provided").red());
        return;
    }

    let command = context.args.first().unwrap();
    match command.as_str() {
        "find_url" => {
            if context.args.len() < 2 {
                println!("{}", style("No URL provided").red());
            } else {
                let url = &context.args[1];
                let downloader = find_for_url(url);

                match downloader {
                    FindResult::InvalidUrl => show_error("Invalid URL"),
                    FindResult::UnrecognizedUrl => show_error("Unrecognized URL"),
                    FindResult::Found(d) => io::show_info(
                        emoji::for_sucess(),
                        format!("{} {}", "The URL matches the downloader for", d.name()),
                    ),
                }
            }
        }
        "download_page" => {
            if context.args.len() < 2 {
                println!("{}", style("No URL provided").red());
            } else {
                let url = &context.args[1];
                io::show_info(emoji::for_fetching(), format!("Retrieving {}", url));

                let response = download_page(url);

                match response {
                    DownloadResult::Unknown(error) => show_error(error.to_string()),
                    DownloadResult::NotFound => show_error("404"),
                    DownloadResult::HttpError(code) => show_error(format!("Got a {} code", code)),
                    DownloadResult::StringTransformationError => {
                        show_error("Error transforming the response into a string")
                    }
                    DownloadResult::Success(string) => io::show_success(
                        emoji::for_sucess(),
                        format!("Cool, it worked. \n {}", string),
                    ),
                }
            }
        }
        _ => {
            println!("{}", style("Unrecognized command").red());
        }
    }
}

fn show_error<S: Into<String>>(error: S) {
    io::show_error(emoji::for_error(), error);
}
