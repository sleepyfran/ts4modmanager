use console::style;
use seahorse::{Command, Context};

/// Creates a command that uninstall a previously installed mod.
pub fn create() -> Command {
    Command::new("uninstall")
        .alias("rm")
        .usage("[name] Removes a previously installed mod. Use list to show everything that is currently installed")
        .action(handler)
}

fn handler(_: &Context) {
    println!("{}", style("Not available yet").red())
}
