// Std Imports
use std::{io::Read, sync::Arc, vec};

// Library Imports
use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;

use crate::{
    lexer::{
        parser::{self, Parser},
        tokenizer::tokenizer,
    },
    utils::App,
    utils::VERSION,
};

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
        let filename: &str = args[0].as_str();
        let mut file =
            std::fs::File::open(filename).unwrap_or_else(|e| app.error(e.to_string().as_str()));
        let mut f_contents = String::new();
        file.read_to_string(&mut f_contents)
            .unwrap_or_else(|e| app.error(e.to_string().as_str()));
        let tokenize = tokenizer(&f_contents);
        let mut parse = parser::Parser::new(tokenize, app);
        println!("{:#?}", parse.parse());
        Ok(())
    }
}
