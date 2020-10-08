mod commands;
mod emoji;
mod io;

use seahorse::App;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("The Sims 4 Mod Manager")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("ts4mods [command] [--flags]")
        .command(commands::install::create())
        .command(commands::list::create())
        .command(commands::uninstall::create())
        .command(commands::update::create());

    #[cfg(debug_assertions)]
    let app = { app.command(commands::debug::create()) };

    app.run(args);
}
