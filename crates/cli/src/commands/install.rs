use console::style;
use seahorse::{Command, Context};

/// Creates a command that install a mod.
pub fn create() -> Command {
    Command::new("install")
        .alias("i")
        .usage("[url] Installs a mod from a URL")
        .action(handler)
}

fn handler(context: &Context) {
    if !context.args.is_empty() {
        println!("{}", style("Not available yet").red())
    } else {
        println!("{}", style("No URL given").red());
        println!("Usage: ts4mods install https://test.com/mod")
    }
}
