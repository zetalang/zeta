mod commands;
mod utils;
// mod errors;
use std::process::exit;
// use errors::Error;
use colored::Colorize;
use commands::AppCommand;
use utils::App;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let app = App::initialize();
    let mut cmd = AppCommand::current().unwrap_or(AppCommand::Unknown); // Default command is help\
    if !app.args.is_empty() {
        cmd = AppCommand::current().unwrap_or(AppCommand::Unknown); // Default command is help
    } else {
        cmd = AppCommand::current().unwrap_or(AppCommand::Help);
    }

    if app.has_flag(&["--help", "-h"]) {
        println!("{}", cmd.help());
        return Ok(());
    }
    if app.has_flag(&["--version", "-v"]) {
        println!("torqc v{}", utils::VERSION.bright_green().bold());
        exit(0);
    }
    cmd.run(app).await?;

    Ok(())
}
