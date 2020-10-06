use console::style;
use seahorse::{Command, Context};

/// Creates a command that lists all installed mods.
pub fn create() -> Command {
    Command::new("list")
        .alias("ls")
        .usage("Lists all mods installed currently")
        .action(handler)
}

fn handler(_: &Context) {
    println!("{}", style("No mods installed yet").red())
}
