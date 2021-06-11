use lazy_static::lazy_static;
use colored::Colorize;
use std::{env, path::PathBuf};

lazy_static! {
	pub static ref ERROR_TAG: String = "error".red().bold().to_string();
}

#[derive(Debug)]
pub struct App {
    pub current_dir: PathBuf,
    pub args: Vec<String>,
    pub flags: Vec<String>,
}

impl App {
    pub fn initialize() -> Self {
        let current_dir = env::current_dir().unwrap();

        let cli_args: Vec<_> = std::env::args().collect();
        let mut args: Vec<String> = Vec::new();
        let mut flags: Vec<String> = Vec::new();

        for arg in cli_args.into_iter().skip(1) {
            if arg.starts_with("--") || arg.starts_with('-') {
                flags.push(arg);
            } else {
                args.push(arg);
            }
        }

        App {
            current_dir,
            args,
            flags,
        }
    }

    pub fn has_flag(&self, flags: &[&str]) -> bool {
        self.flags
            .iter()
            .any(|flag| flags.iter().any(|search_flag| flag == search_flag))
    }
}