// Std Imports
use std::{sync::Arc, vec};

// Library Imports
use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;

use crate::{utils::App, utils::VERSION};

// Super Imports
use super::Command;

pub struct Compile {}

#[async_trait]
impl Command for Compile {
    fn help() -> String {
        format!(
            r#"torqc {}
    
Usage: {} {} {}
Commands:
  {asterisk} {} - Compiles the given file
Flags: 
  {asterisk} {} - uses clang (default)
  {asterisk} {} - uses gcc
  {asterisk} {} - verbose output
  {asterisk} {} - Builds for deployement
  "#,
            VERSION.bright_green().bold(),
            "torqc".bright_green().bold(),
            "[commands]".bright_purple(),
            "[flags]".bright_purple(),
            "<filename>".bright_blue(),
            "--useclang, -ucg".bright_blue(),
            "--usegcc, -ugcc ".bright_blue(),
            "--verbose, -vb  ".bright_blue(),
            "--release, -r   ".bright_blue(),
            asterisk = "*".bright_magenta().bold(),
        )
    }

    async fn exec(app: Arc<App>) -> Result<()> {
        let acceptedflags: Vec<&str> = vec![
            "--useclang",
            "-ucg",
            "--usegcc",
            "-ugcc",
            "--verbose",
            "-vb",
            "--release",
            "-r",
        ];
        let flags = app.filter_flag(&acceptedflags);

        let args = app.args.clone();
        let command: &str = args[0].as_str();
        // println!("{:#?} {:#?}", flags, app.flags);
        Ok(())
    }
}
