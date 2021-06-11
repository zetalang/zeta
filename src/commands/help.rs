use std::sync::Arc;

// Library Imports
use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;

// Crate Level Imports
use crate::utils::App;
use crate::VERSION;

// Super Imports
use super::Command;

/// Struct implementation for the `Help` command.
pub struct Help;

#[async_trait]
impl Command for Help {
    fn help() -> String {
        format!(
            r#"torqc {}
    
Displays help information.
Usage: {} {} {}
Commands:
  {} {} - Compiles the given file 
  "#,
            VERSION.bright_green().bold(),
            "torqc".bright_green().bold(),
            "[commands]".bright_purple(),
            "[flags]".bright_purple(),
            "*".bright_magenta().bold(),
            "<filename>".bright_blue(),
        )
    }

    async fn exec(_app: Arc<App>) -> Result<()> {
        println!("{}", Self::help());
        Ok(())
    }
}
