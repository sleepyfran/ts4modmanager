use console::style;
use seahorse::{Command, Context};

/// Creates a command that updates all mods or a certain one.
pub fn create() -> Command {
    Command::new("update")
        .alias("u")
        .usage("[name] Updates the mod with the given name. If called with no arguments, updates all mods")
        .action(handler)
}

fn handler(_: &Context) {
    println!("{}", style("Not available yet").red())
}
