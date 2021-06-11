mod utils;
mod commands;
// mod errors;
use std::{process::exit, time::Instant};
// use errors::Error;
use colored::Colorize;
use utils::App;
use commands::AppCommand;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let app = App::initialize();
    let mut cmd = AppCommand::current().unwrap_or(AppCommand::Unknown); // Default command is help\
   if app.args.len() > 0 {
        cmd = AppCommand::current().unwrap_or(AppCommand::Unknown); // Default command is help
    }else{
        cmd = AppCommand::current().unwrap_or(AppCommand::Help);
    }

    if app.has_flag(&["--help", "-h"]) {
        println!("{}", cmd.help());
        return Ok(());
    }
    if app.has_flag(&["--version", "-v"]) {
        println!(
            "torqc v{}",
            VERSION.bright_green().bold()
        );
        exit(0);
    }
    let start = Instant::now();
    cmd.run(app).await?;
    let end = Instant::now();
    println!("Finished in {:.2}s", (end - start).as_secs_f32());

    Ok(())
}
