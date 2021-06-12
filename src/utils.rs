use colored::Colorize;
use lazy_static::lazy_static;
use std::{env, path::PathBuf, process::exit};

lazy_static! {
    pub static ref ERROR_TAG: String = "error".red().bold().to_string();
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[derive(Debug)]
pub struct App {
    pub current_dir: PathBuf,
    pub args: Vec<String>,
    pub flags: Vec<String>,
}

impl App {
    pub fn initialize() -> Self {
        console::colors_enabled();
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
    pub fn error(&self, error: &str) -> ! {
        eprintln!(
            "{}: {}\n  {}",
            "Error".red().bold(),
            "Caused By".truecolor(190, 190, 190).italic().bold(),
            error.truecolor(195, 195, 195).bright_white()
        );
        exit(0)
    }
    pub fn has_flag(&self, flags: &[&str]) -> bool {
        self.flags
            .iter()
            .any(|flag| flags.iter().any(|search_flag| flag == search_flag))
    }

    pub fn filter_flag(&self, accepted_flags_arg: &[&str]) -> Vec<String> {
        let accepted_flags: Vec<String> = self
            .flags
            .iter()
            .filter(|item| accepted_flags_arg.contains(&item.as_str()))
            .cloned()
            .collect();
        if accepted_flags.len() != self.flags.len() {
            let s: Vec<String> = self
                .flags
                .iter()
                .filter(|item| !accepted_flags_arg.contains(&item.as_str()))
                .cloned()
                .collect();
            let mut i: String = String::new();
            for (l, val) in s.iter().enumerate() {
                if l != 0 {
                    i = i + ", " + val;
                } else {
                    i = val.clone();
                }
            }
            println!("{}: Not a valid flag {}", "Warning".yellow().bold(), i);
        }
        accepted_flags
    }
}
