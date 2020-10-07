use console::style;
use seahorse::{Command, Context};

use core::downloaders::{find_for_url, FindResult};

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
                    FindResult::InvalidUrl => println!("{}", style("Invalid URL").red()),
                    FindResult::UnrecognizedUrl => println!("{}", style("Unrecognized URL").red()),
                    FindResult::Found(d) => {
                        println!(
                            "{} {}",
                            style("The URL matches the downloader for").green(),
                            d.name()
                        );
                    }
                }
            }
        }
        _ => {
            println!("{}", style("Unrecognized command").red());
        }
    }
}
